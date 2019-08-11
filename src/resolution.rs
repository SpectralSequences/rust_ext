use std::cmp::{min, max};
use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Mutex;

use bivec::BiVec;

use crate::fp_vector::{FpVector, FpVectorT};
use crate::matrix::{Matrix, Subspace};
use crate::algebra::{Algebra, AlgebraAny};
use crate::module::{Module, OptionModule};
use crate::free_module::FreeModule;
use crate::once::{OnceVec, OnceBiVec, TempStorage};
use crate::finite_dimensional_module::FiniteDimensionalModule as FDModule;
use crate::module_homomorphism::{ModuleHomomorphism, ZeroHomomorphism};
use crate::free_module_homomorphism::FreeModuleHomomorphism;
use crate::chain_complex::ChainComplex;
use crate::chain_complex::ChainComplexConcentratedInDegreeZero as CCDZ;
use crate::resolution_homomorphism::{ResolutionHomomorphism, ResolutionHomomorphismToUnit};

struct Cocycle {
    s : u32,
    t : i32,
    index : usize,
    name : String
}

struct SelfMap<
    M : Module, F : ModuleHomomorphism<M, M>, CC : ChainComplex<M, F>
> {
    s : u32,
    t : i32,
    name : String,
    map_data : TempStorage<Matrix>,
    map : ResolutionHomomorphism<M, F, CC, M, F, CC>
}

/// #Fields
///  * `kernels` - For each *internal* degree, store the kernel of the most recently calculated
///  chain map as returned by `generate_old_kernel_and_compute_new_kernel`, to be used if we run
///  resolve_through_degree again.
pub struct Resolution<M : Module, F : ModuleHomomorphism<M, M>, CC : ChainComplex<M, F>> {
    complex : Rc<CC>,
    modules : OnceVec<Rc<FreeModule>>,
    zero_module : Rc<FreeModule>,
    chain_maps : OnceVec<FreeModuleHomomorphism<M>>,
    differentials : OnceVec<FreeModuleHomomorphism<FreeModule>>,
    phantom : PhantomData<ChainComplex<M, F>>,

    pub kernels : OnceBiVec<RefCell<Option<Subspace>>>,

    pub next_s : Mutex<u32>,
    pub next_t : Mutex<i32>,
    pub add_class : Option<Box<dyn Fn(u32, i32, &str)>>,
    pub add_structline : Option<Box<dyn Fn(
        &str,
        u32, i32, usize, 
        u32, i32, usize
    )>>,

    // Products
    unit_resolution : Option<Rc<RefCell<ModuleResolution<FDModule>>>>,
    product_list : Vec<Cocycle>,
    // s -> t -> idx -> resolution homomorphism to unit resolution. We don't populate this
    // until we actually have a unit resolution, of course.
    chain_maps_to_unit_resolution : OnceVec<OnceBiVec<OnceVec<ResolutionHomomorphismToUnit<M, F, CC>>>>,
    max_product_homological_degree : u32,

    // Self maps
    self_maps : Vec<SelfMap<M, F, CC>>
}

impl<M : Module, F : ModuleHomomorphism<M, M>, CC : ChainComplex<M, F>> Resolution<M, F, CC> {
    pub fn new(
        complex : Rc<CC>,
        add_class : Option<Box<dyn Fn(u32, i32, &str)>>,
        add_structline : Option<Box<dyn Fn(
            &str,
            u32, i32, usize, 
            u32, i32, usize
        )>>
    ) -> Self {
        let algebra = complex.get_algebra();
        let min_degree = complex.get_min_degree();

        let zero_module = Rc::new(FreeModule::new(Rc::clone(&algebra), "F_{-1}".to_string(), min_degree));

        Self {
            complex,
            chain_maps : OnceVec::new(),

            modules : OnceVec::new(),
            zero_module,
            differentials : OnceVec::new(),
            kernels : OnceBiVec::new(min_degree),
            phantom : PhantomData,

            next_s : Mutex::new(0),
            next_t : Mutex::new(min_degree),
            add_class,
            add_structline,

            chain_maps_to_unit_resolution : OnceVec::new(),
            max_product_homological_degree : 0,
            product_list : Vec::new(),
            unit_resolution : None,

            self_maps : Vec::new()
        }
    }

    pub fn get_max_degree(&self) -> i32 {
        *self.next_t.lock().unwrap() - 1
    }

    pub fn get_max_hom_deg(&self) -> u32 {
        *self.next_s.lock().unwrap() - 1
    }
    
    pub fn get_complex(&self) -> Rc<CC> {
        Rc::clone(&self.complex)
    }

    pub fn get_module(&self, homological_degree : u32) -> Rc<FreeModule> {
        Rc::clone(&self.modules[homological_degree as usize])
    }

    pub fn get_number_of_gens_in_bidegree(&self, homological_degree : u32, internal_degree : i32) -> usize {
        self.get_module(homological_degree).get_number_of_gens_in_degree(internal_degree)
    }

    pub fn get_chain_map(&self, homological_degree : u32) -> &FreeModuleHomomorphism<M> {
        &self.chain_maps[homological_degree as usize]
    }

    pub fn get_cocycle_string(&self, hom_deg : u32, int_deg : i32, idx : usize) -> String {
        let p = self.prime();
        let d = self.get_differential(hom_deg);
        let source = self.get_module(hom_deg);
        let target = d.get_target();
        let dimension = target.get_dimension(int_deg);
        let basis_idx = source.operation_generator_to_index(0, 0, int_deg, idx);
        let mut result_vector = crate::fp_vector::FpVector::new(p, dimension);
        d.apply_to_basis_element(&mut result_vector, 1, int_deg, basis_idx);
        return target.element_to_string(int_deg, &result_vector);
    }

    /// This function prepares the Resolution object to perform computations up to the specified
    /// s degree. It does *not* perform any computations by itself. It simply lengthens the
    /// `OnceVec`s `modules`, `chain_maps`, etc. to the right length.
    pub fn extend_through_degree(&self, mut next_s : u32, max_s : u32, next_t : i32, max_t : i32) {
        let min_degree = self.get_min_degree();

        for i in next_s ..= max_s {
            self.modules.push(Rc::new(FreeModule::new(Rc::clone(&self.get_algebra()), format!("F{}", i), min_degree)));
            self.chain_maps.push(FreeModuleHomomorphism::new(Rc::clone(&self.modules[i]), Rc::clone(&self.complex.get_module(i)), 0));
        }

        for _ in next_t ..= max_t {
            self.kernels.push(RefCell::new(None));
        }

        if next_s == 0 {
            self.differentials.push(FreeModuleHomomorphism::new(Rc::clone(&self.modules[0u32]), Rc::clone(&self.zero_module), 0));
            next_s += 1;
        }
        for i in next_s ..= max_s {
            self.differentials.push(FreeModuleHomomorphism::new(Rc::clone(&self.modules[i]), Rc::clone(&self.modules[i - 1]), 0));
        }
    }

    pub fn resolve_through_bidegree(&self, self_: &Rc<RefCell<Self>>, mut max_s : u32, mut max_t : i32) {
        let min_degree = self.get_min_degree();
        let mut next_s = self.next_s.lock().unwrap();
        let mut next_t = self.next_t.lock().unwrap();

        // We want the computed area to always be a square.
        max_t = max(max_t, *next_t - 1);
        if max_s < *next_s {
            max_s = *next_s - 1;
        }

        self.extend_through_degree(*next_s, max_s, *next_t, max_t);
        self.get_algebra().compute_basis(max_t);// because Adem has off-by-one

        if let Some(unit_res) = &self.unit_resolution {
            unit_res.borrow().resolve_through_bidegree(&unit_res, self.max_product_homological_degree, max_t);
        }

        for t in min_degree ..= max_t {
            // TODO: Just use the borrow_mut instead of cloning
            let mut new_kernel = self.kernels[t].borrow_mut().clone();

            let start = if t < *next_t { *next_s } else { 0 };
            for s in start ..= max_s {
                new_kernel = Some(self.step(&self_, s as u32, t, new_kernel));
            }
            *self.kernels[t].borrow_mut() = new_kernel;
        }

        *next_s = max_s + 1;
        *next_t = max_t + 1;
    }

    // We cannot do self : Rc<RefCell<Self>>. We should probably check that self_ and &self
    // indeed refer to the same thing...
    pub fn resolve_through_degree(&self, self_: &Rc<RefCell<Self>>, degree : i32) {
        self.resolve_through_bidegree(self_, degree as u32, degree);
    }

    pub fn step(&self, self_: &Rc<RefCell<Self>>, s : u32, t : i32, old_kernel : Option<Subspace>) -> Subspace {
        // println!("step : hom_deg : {}, int_deg : {}", homological_degree, degree);
        if s == 0 {
            self.zero_module.extend_by_zero(t);
        }
        self.get_complex().compute_through_bidegree(s, t);
        let new_kernel = self.generate_old_kernel_and_compute_new_kernel(s, t, old_kernel);
        let module = self.get_module(s);
        let num_gens = module.get_number_of_gens_in_degree(t);
        if let Some(f) = &self.add_class {
            for i in 0..num_gens {
                f(s, t, &format!("{}", i));
            }
        }
        if let Some(_) = &self.add_structline {
            for i in 0..num_gens {
                self.compute_filtration_one_products(s, t, i);
            }
        }
        self.extend_maps_to_unit(self_, s, t);
        self.compute_products(s, t);
        self.compute_self_maps(s, t);
        new_kernel
    }

    fn compute_filtration_one_products(&self, homological_degree : u32, internal_degree : i32, source_idx : usize){
        if homological_degree == 0 {
            return;
        }
        if let Some(add_structline) = &self.add_structline {
            let d = self.get_differential(homological_degree);
            let target = self.get_module(homological_degree - 1);
            let dx = d.get_output(internal_degree, source_idx);
            for (op_name, op_degree, op_index) in self.get_algebra().get_filtration_one_products() {
                let gen_degree = internal_degree - *op_degree;

                if gen_degree < self.get_min_degree(){
                    break;
                }

                let num_target_generators = target.get_number_of_gens_in_degree(gen_degree);
                for target_idx in 0 .. num_target_generators {
                    let vector_idx = target.operation_generator_to_index(*op_degree, *op_index, gen_degree, target_idx);
                    if vector_idx >= dx.get_dimension() {
                        // println!("Out of bounds index when computing product:");
                        // println!("  ==  degree: {}, hom_deg: {}, dim: {}, idx: {}", degree, homological_degree, dx.dimension, vector_idx);
                    } else {
                        // printf("hom_deg: %d, deg: %d, source_idx: %d, op_deg: %d, entry: %d\n", homological_degree, degree, source_idx, op_degree, Vector_getEntry(dx, vector_idx));
                        if dx.get_entry(vector_idx) != 0 {
                            // There was a product!
                            add_structline(op_name, homological_degree - 1, gen_degree, target_idx, homological_degree, internal_degree, source_idx);
                        }
                    }
                }
            }
        }
    }

    pub fn add_structline(
            &self, 
            name : &str,
            source_hom_deg : u32, source_int_deg : i32, source_idx : usize, 
            target_hom_deg : u32, target_int_deg : i32, target_idx : usize
    ){
        if let Some(add_structline) = &self.add_structline {
            add_structline(name, source_hom_deg, source_int_deg, source_idx, target_hom_deg, target_int_deg, target_idx);
        }
    }

    // pub fn set_empty(&self, homological_degree : u32, degree : i32){
    //     let current_differential = self.get_differential(homological_degree);
    //     let source = current_differential.source;
    //     let source_module_table = source.construct_table(degree);
    // }

    /// Call our resolution $X$, and the chain complex to resolve $C$. This is a legitimate
    /// resolution if the map $f: X \to C$ induces an isomorphism on homology. This is the same as
    /// saying the cofiber is exact. The cofiber is given by the complex
    ///
    /// $$ X_s \oplus C_{s+1} \to X_{s-1} \oplus C_s \to X_{s-2} \oplus C_{s-1} \to \cdots $$
    ///
    /// where the differentials are given by
    ///
    /// $$ \begin{pmatrix} d_X & 0 \\\\ (-1)^s f & d_C \end{pmatrix} $$
    ///
    /// Our method of producing $X_{s, t}$ and the chain maps are as follows. Suppose we have already
    /// built the chain map and differential for $X_{s-1, t}$ and $X_{s, t-1}$. Since $X_s$ is a
    /// free module, the generators in degree $< t$ gives us a bunch of elements in $X_s$ already,
    /// and we know exactly where they get mapped to. Let $T$ be the $\\mathbb{F}_p$ vector space
    /// generated by these elements. Then we already have a map
    ///
    /// $$ T \to X_{s-1, t} \oplus C_{s, t}$$
    ///
    /// and we know this hits the kernel of the map
    ///
    /// $$ D = X_{s-1, t} \oplus C_{s, t} \to X_{s-2, t} \oplus C_{s-1, t}. $$
    ///
    /// What we need to do now is to add generators to $X_{s, t}$ to hit the entirity of this
    /// kernel.  Note that we don't *have* to do this. Some of the elements in the kernel might be
    /// hit by $C_{s+1, t}$ and we don't have to hit them, but we opt to add generators to hit it
    /// anyway.
    ///
    /// If we do it this way, then we know the composite of the map
    ///
    /// $$ T \to X_{s-1, t} \oplus C_{s, t} \to C_{s, t} $$
    ///
    /// has to be surjective, since the image of $C_{s, t}$ under $D$ is also in the image of $X_{s-1, t}$.
    /// So our first step is to add generators to $X_{s, t}$ such that this composite is
    /// surjective.
    ///
    /// After adding these generators, we need to decide where to send them to. We know their
    /// values in the $C_{s, t}$ component, but we need to use a quasi-inverse to find the element in
    /// $X_{s-1, t}$ that hits the corresponding image of $C_{s, t}$. This tells us the $X_{s-1,
    /// t}$ component.
    ///
    /// Finally, we need to add further generators to $X_{s, t}$ to hit all the elements in the
    /// kernel of
    ///
    /// $$ X_{s-1, t} \to X_{s-2, t} \oplus C_{s-1, t}. $$
    ///
    /// This kernel was recorded by the previous iteration of the method in `old_kernel`, so this
    /// step is doable as well.
    ///
    /// Note that if we add our new generators conservatively, then the kernel of the maps
    ///
    /// $$
    /// \begin{aligned}
    /// T &\to X_{s-1, t} \oplus C_{s, t} \\\\
    /// X_{s, t} &\to X_{s-1, t} \oplus C_{s, t}
    /// \end{aligned}
    /// $$
    /// agree.
    ///
    /// In the code, we first row reduce the matrix of the map from $T$. This lets us record
    /// the kernel which is what the function returns at the end. This computation helps us perform
    /// the future steps since we need to know about the cokernel of this map.
    ///
    /// # Arguments
    ///  * `homological_degree` - The s degree to calculate
    ///  * `degree` - The t degree to calculate
    ///  * `old_kernel` - The kernel of the map $X_{s-1, t} \to X_{s-2, t} \oplus C_{s-1, t}$, computed
    ///  and returned by the previous iteration of this function for $(s-1, t)$. This is `None` when $s = 0$.
    pub fn generate_old_kernel_and_compute_new_kernel(&self, homological_degree : u32, degree : i32, old_kernel : Option<Subspace>) -> Subspace {
        // println!("====hom_deg : {}, int_deg : {}", homological_degree, degree);
        let p = self.prime();
        //                           current_chain_map
        //                X_{s, t} --------------------> C_{s, t}
        //                   |                               |
        //                   | current_differential          |
        //                   v                               v
        // old_kernel <= X_{s-1, t} -------------------> C_{s-1, t}

        let current_differential = self.get_differential(homological_degree);
        let current_chain_map = self.get_chain_map(homological_degree);
        let complex = self.get_complex();
        let complex_cur_differential = complex.get_differential(homological_degree);
        let source = &current_differential.get_source();
        let target_cc = &current_chain_map.get_target();
        let target_res = &current_differential.get_target();
        let (source_lock, source_module_table) = source.construct_table(degree);
        let mut chain_map_lock = current_chain_map.get_lock();
        let mut differential_lock = current_differential.get_lock();
        let source_dimension = FreeModule::get_dimension_with_table(&source_module_table);
        let target_cc_dimension = target_cc.get_dimension(degree);
        let target_res_dimension = target_res.get_dimension(degree);
        let target_dimension = target_cc_dimension + target_res_dimension;
        // The Homomorphism matrix has size source_dimension x target_dimension, but we are going to augment it with an
        // identity matrix so that gives a matrix with dimensions source_dimension x (target_dimension + source_dimension).
        // Later we're going to write into this same matrix an isomorphism source/image + new vectors --> kernel
        // This has size target_dimension x (2*target_dimension).
        // This latter matrix may be used to find a preimage of an element under the differential.

        // Pad the target dimension so that it ends in an aligned position.
        let padded_target_cc_dimension = FpVector::get_padded_dimension(p, target_cc_dimension);
        let padded_target_res_dimension = FpVector::get_padded_dimension(p, target_res_dimension);
        let padded_target_dimension = padded_target_res_dimension + padded_target_cc_dimension;
        let rows = source_dimension + target_dimension;
        let columns = padded_target_dimension + source_dimension + rows;
        let mut matrix = Matrix::new(p, rows, columns);
        let mut pivots = vec![-1;matrix.get_columns()];
        matrix.set_slice(0, source_dimension, 0, padded_target_dimension + source_dimension);
        // Get the map (d, f) : X_{s, t} -> X_{s-1, t} (+) C_{s, t} into matrix
        current_chain_map.get_matrix_with_table(&mut matrix, &source_module_table, degree, 0, 0);
        current_differential.get_matrix_with_table(&mut matrix, &source_module_table, degree, 0, padded_target_cc_dimension);
        // Augment with the identity matrix.
        for i in 0 .. source_dimension {
            matrix[i].set_entry(padded_target_dimension + i, 1);
        }
        matrix.row_reduce(&mut pivots);

        let new_kernel = matrix.compute_kernel(&pivots, padded_target_dimension);
        let kernel_rows = new_kernel.matrix.get_rows();
        let first_new_row = source_dimension;
        matrix.clear_slice();

        // Now add generators to surject onto C_{s, t}.
        // (For now we are just adding the eventual images of the new generators into matrix, we will update
        // X_{s,t} and f later).
        // We record which pivots exactly we added so that we can walk over the added genrators in a moment and
        // work out what dX should to to each of them.
        let new_generators = matrix.extend_to_surjection(first_new_row, 0, target_cc_dimension, &pivots);
        let mut num_new_gens = new_generators.len();

        if homological_degree > 0 {
            // Now we need to make sure that we have a chain homomorphism. Each generator x we just added to 
            // X_{s,t} has a nontrivial image f(x) \in C_{s,t}. We need to set d(x) so that f(dX(x)) = dC(f(x)).
            // So we set dX(x) = f^{-1}(dC(f(x)))
            let prev_chain_map = self.get_chain_map(homological_degree - 1);
            let maybe_quasi_inverse = prev_chain_map.get_quasi_inverse(degree);
            if let Some(quasi_inverse) = maybe_quasi_inverse {
                let mut out_vec = FpVector::new(self.prime(), target_res_dimension);
                let dfx_dim = complex_cur_differential.get_target().get_dimension(degree);
                let mut dfx = FpVector::new(self.prime(), dfx_dim);
                for (i, column) in new_generators.iter().enumerate() {
                    complex_cur_differential.apply_to_basis_element(&mut dfx, 1, degree, *column);
                    quasi_inverse.apply(&mut out_vec, 1, &dfx);
                    // Now out_vec contains f^{-1}(dC(f(x))).
                    let out_row = &mut matrix[first_new_row + i];
                    let old_slice = out_row.get_slice();
                    // dX(x) goes into the column range [padded_target_cc_dimension, padded_target_cc_dimension + target_res_dimension] in the matrix
                    // I think we are missing a sign here.
                    out_row.set_slice(padded_target_cc_dimension, padded_target_cc_dimension + target_res_dimension);
                    out_row.assign(&out_vec);
                    out_row.restore_slice(old_slice);
                    dfx.set_to_zero();
                    out_vec.set_to_zero();
                }
                // Row reduce again since our activity may have changed the image of dX.
                if new_generators.len() > 0 {
                    matrix.row_reduce(&mut pivots);
                }
            }
            // Now we add new generators to hit any cycles in old_kernel that we don't want in our homology.
            num_new_gens += matrix.extend_image(first_new_row + num_new_gens, padded_target_cc_dimension, padded_target_cc_dimension + target_res_dimension, &pivots, old_kernel).len();
        }
        source.add_generators(degree, source_lock, source_module_table, num_new_gens, None);
        current_chain_map.add_generators_from_matrix_rows(&chain_map_lock, degree, &mut matrix, first_new_row, 0, num_new_gens);
        current_differential.add_generators_from_matrix_rows(&differential_lock, degree, &mut matrix, first_new_row, padded_target_cc_dimension, num_new_gens);

        // Record the quasi-inverses for future use.
        // The part of the matrix that contains interesting information is occupied_rows x (target_dimension + source_dimension + kernel_size).
        let image_rows = first_new_row + num_new_gens;
        // println!("{}",matrix);
        for i in first_new_row .. image_rows {
            matrix[i].set_entry(padded_target_dimension + i, 1);
        }
        // println!("{}",matrix);
        matrix.set_slice(0, image_rows, 0, padded_target_dimension + source_dimension + num_new_gens); 
        let mut new_pivots = vec![-1;matrix.get_columns()];
        matrix.row_reduce(&mut new_pivots);
        let (cm_qi, res_qi) = matrix.compute_quasi_inverses(
            &new_pivots, 
            padded_target_cc_dimension, 
            padded_target_cc_dimension + target_res_dimension,
            padded_target_dimension
        );
        // assert!(res_qi)

        current_chain_map.set_quasi_inverse(&chain_map_lock, degree, cm_qi);
        current_differential.set_quasi_inverse(&differential_lock, degree, res_qi);
        *chain_map_lock += 1;
        *differential_lock += 1;

        new_kernel
    }

    pub fn graded_dimension_string(&self) -> String {
        let mut result = String::new();
        let min_degree = self.get_min_degree();
        let max_degree = self.get_max_degree();
        let max_hom_deg = self.get_max_hom_deg(); //(max_degree - min_degree) as u32 / (self.prime() + 1); //self.get_max_hom_deg();
        for i in (0 ..= max_hom_deg).rev() {
            let module = self.get_module(i);
            for j in min_degree + i as i32 ..= max_degree {
                let n = module.get_number_of_gens_in_degree(j);
                match n {
                    0 => result.push_str("  "),
                    1 => result.push_str("· "),
                    2 => result.push_str(": "),
                    3 => result.push_str("∴ "),
                    4 => result.push_str("⁘ "),
                    5 => result.push_str("⁙ "),
                    _ => result.push_str(&format!("{} ", n))
                }
            }
            result.push_str("\n");
            // If it is empty so far, don't print anything
            if result.trim_start().is_empty() {
                result = String::new();
            }
        }
        return result;
    }

}

// Product algorithms
impl<M, F, CC> Resolution<M, F, CC> where
    M : Module,
    F : ModuleHomomorphism<M, M>,
    CC : ChainComplex<M, F>
{
    pub fn add_product(&mut self, s : u32, t : i32, index : usize, name : String) {
         if self.unit_resolution.is_none() {
             self.construct_unit_resolution();
         }
         if s > self.max_product_homological_degree {
             self.max_product_homological_degree = s;
         }
         self.product_list.push(Cocycle { s, t, index, name });
    }

    pub fn construct_unit_resolution(&mut self) {
         let unit_module = Rc::new(FDModule::new(self.get_algebra(), String::from("unit"), BiVec::from_vec(0, vec![1])));
         let ccdz = Rc::new(CCDZ::new(unit_module));
         self.unit_resolution = Some(Rc::new(RefCell::new(Resolution::new(ccdz, None, None))));
    }

    pub fn set_unit_resolution(&mut self, unit_res : Rc<RefCell<ModuleResolution<FDModule>>>) {
        if self.chain_maps_to_unit_resolution.len() > 0 {
            panic!("Cannot change unit resolution after you start computing products");
        }
        self.unit_resolution = Some(unit_res);
    }

    /// Compute products whose result lie in degrees up to (s, t)
    fn compute_products(&self, s : u32, t : i32) {
        for elt in &self.product_list {
            if s < elt.s || t < self.get_min_degree() + elt.t {
                continue;
            }

            let source_s = s - elt.s;
            let source_t = t - elt.t;

            for k in 0.. self.get_number_of_gens_in_bidegree(source_s, source_t) {
                self.compute_product_step(elt, source_s, source_t, k);
            }
        }
    }

    fn compute_product_step(&self, elt : &Cocycle, s : u32, t : i32, idx : usize)
    {
        let f = &self.chain_maps_to_unit_resolution[s][t][idx];
        let target_s = s + elt.s;
        let target_t = t + elt.t;

        let unit_res = self.unit_resolution.as_ref().unwrap().borrow();
        let output_module = unit_res.get_module(elt.s);

        let mut result = FpVector::new(self.prime(), output_module.get_dimension(elt.t));

        for l in 0 .. self.get_number_of_gens_in_bidegree(target_s, target_t) {
            f.get_map(elt.s).apply_to_generator(&mut result, 1, target_t, l);

            let vector_idx = output_module.operation_generator_to_index(0, 0, elt.t, elt.index);
            if result.get_entry(vector_idx) != 0 {
                self.add_structline(&elt.name, s, t, idx, target_s, target_t, l);
            }
            result.set_to_zero();
        }
    }

    /// This ensures the chain_maps_to_unit_resolution are defined such that we can compute products up
    /// to bidegree (s, t)
    fn extend_maps_to_unit(&self, self_ : &Rc<RefCell<Self>>, s : u32, t : i32) {
        // If there are no products, we return
        if self.product_list.len() == 0 {
            return;
        }

        let p = self.prime();
        let s_idx = s as usize;

        // Now we populate the arrays if the ResolutionHomomorphisms have not been defined.
        let num_gens = self.get_module(s).get_number_of_gens_in_degree(t);
        if t == self.get_min_degree() {
            assert!(s_idx == self.chain_maps_to_unit_resolution.len());
            self.chain_maps_to_unit_resolution.push(OnceBiVec::new(self.get_min_degree()));
        } else {
            assert!(s_idx < self.chain_maps_to_unit_resolution.len());
        }
        assert!(self.chain_maps_to_unit_resolution[s_idx].len() == t);

        self.chain_maps_to_unit_resolution[s_idx].push(OnceVec::new());
        if num_gens > 0 {
            let mut unit_vector = Matrix::new(p, num_gens, 1);
            for j in 0 .. num_gens {
                let f = ResolutionHomomorphism::new(
                    format!("(hom_deg : {}, int_deg : {}, idx : {})", s, t, j),
                    Rc::downgrade(self_), Rc::downgrade(self.unit_resolution.as_ref().unwrap()),
                    s, t
                );
                unit_vector[j].set_entry(0, 1);
                f.extend_step(s, t, Some(&mut unit_vector));
                unit_vector[j].set_to_zero();
                self.chain_maps_to_unit_resolution[s_idx][t].push(
                    f
                )
            }
        }

        // Now we actually extend the maps.
        let max_hom_deg = min(s, self.max_product_homological_degree);
        let min_degree = self.get_min_degree();
        for i in 0 ..= max_hom_deg {
            for j in min_degree ..= t {
                let hom_deg = s - i;
                let num_gens = self.get_module(hom_deg).get_number_of_gens_in_degree(j);
                for k in 0 .. num_gens {
                    let f = &self.chain_maps_to_unit_resolution[hom_deg as usize][j][k];
                    f.extend(s, t);
                }
            }
        }
    }
}

// Self map algorithms
impl<M, F, CC> Resolution<M, F, CC> where
    M : Module,
    F : ModuleHomomorphism<M, M>,
    CC : ChainComplex<M, F>
{
    pub fn add_self_map(&mut self, self_: &Rc<RefCell<Self>>,  s : u32, t : i32, name : String, map_data : Matrix) {
        self.self_maps.push(
            SelfMap {
                s, t, name, map_data : TempStorage::new(map_data),
                map : ResolutionHomomorphism::new("".to_string(), Rc::downgrade(self_), Rc::downgrade(self_), s, t)
            });
    }

    /// We compute the products by self maps where the result has degree (s, t).
    fn compute_self_maps(&self, s : u32, t : i32) {
        let p = self.prime();
        for f in &self.self_maps {
            if s < f.s || t < f.t + self.get_min_degree() {
                continue;
            }
            if s == f.s && t == f.t + self.get_min_degree() {
                let mut map_data = f.map_data.take();
                f.map.extend_step(s, t, Some(&mut map_data));
            }
            f.map.extend(s, t);
//            t -= 1;

            let target_s = s - f.s;
            let target_t = t - f.t;

            let source_module = self.get_module(s);
            let target_module = self.get_module(target_s);

            let num_source_gens = source_module.get_number_of_gens_in_degree(t);
            let num_target_gens = target_module.get_number_of_gens_in_degree(target_t);

            if num_source_gens == 0 || num_target_gens == 0 {
                continue;
            }

            let target_dim = target_module.get_dimension(target_t);
            let mut result = FpVector::new(p, target_dim);

            for j in 0 .. num_source_gens {
                f.map.get_map(target_s).apply_to_generator(&mut result, 1, t, j);
                for k in 0 .. num_target_gens {
                    let vector_idx = target_module.operation_generator_to_index(0, 0, target_t, k);
                    if result.get_entry(vector_idx) != 0 {
                        self.add_structline(
                            &f.name,
                            target_s, target_t, k,
                            s, t, j
                            );
                    }
                }
                result.set_to_zero();
            }
        }
    }
}

impl<M : Module, F : ModuleHomomorphism<M, M>, CC : ChainComplex<M, F>> 
    ChainComplex<FreeModule, FreeModuleHomomorphism<FreeModule>> 
    for Resolution<M, F, CC>
{
    fn get_algebra(&self) -> Rc<AlgebraAny> {
        self.get_complex().get_algebra()
    }

    fn get_module(&self, homological_degree : u32) -> Rc<FreeModule> {
        self.get_module(homological_degree)
    }

    fn get_min_degree(&self) -> i32 {
        self.get_complex().get_min_degree()
    }

    fn get_differential(&self, homological_degree : u32) -> &FreeModuleHomomorphism<FreeModule> {
        &self.differentials[homological_degree as usize]
    }

    // TODO: implement this.
    fn compute_through_bidegree(&self, hom_deg : u32, int_deg : i32) {

    }

    // fn computed_through_bidegree_q(&self, hom_deg : u32, int_deg : i32) -> bool {
    //     self.res_inner.rent(|res_homs| {
    //         res_homs.differentials.len() > hom_deg 
    //             && res_homs.differentials[hom_deg as usize].
    //     })
    // }
}

pub type ModuleResolution<M>
    = Resolution<
        OptionModule<M>, 
        ZeroHomomorphism<OptionModule<M>, OptionModule<M>>, 
        CCDZ<M>
    >;
