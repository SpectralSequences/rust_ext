use std::sync::{Mutex, MutexGuard};

use crate::fp_vector::{FpVector, FpVectorT};
use crate::block_structure::BlockStructure;
use crate::algebra::AlgebraAny;
use crate::field::Field;
use crate::module::Module;
use crate::free_module::FreeModule;
use crate::finite_dimensional_module::FiniteDimensionalModuleT;
use crate::hom_space::HomSpace;
use crate::module_homomorphism::ModuleHomomorphism;
use crate::free_module_homomorphism::FreeModuleHomomorphism;

struct HomPrecomposition<M> {
    source : Rc<HomSpace<M>>,
    target : Rc<HomSpace<M>>,
    map : Rc<FreeModuleHomomorphism<FreeModule>>,
    max_computed_degree : Mutex<i32>,

}

impl<M : FiniteDimensionalModuleT> HomPrecomposition<M> {
    pub fn new(source : Rc<HomSpace<M>>, target : Rc<HomSpace<M>>, map : Rc<FreeModuleHomomorphism<FreeModule>>) -> Self {
        let min_degree = source.min_degree();
        Self {
            source,
            target,
            map,
            max_computed_degree : Mutex::new(min_degree - 1)
        }
    }
}

impl<M : FiniteDimensionalModuleT> ModuleHomomorphism<HomSpace<M>, HomSpace<M>> for HomPrecomposition<M> {
    fn get_source(&self) -> Rc<HomSpace<M>> {
        Rc::clone(&self.source)
    }

    fn get_target(&self) -> Rc<HomSpace<M>> {
        Rc::clone(&self.target)
    }

    fn get_min_degree(&self) -> i32 {
        self.get_source().get_min_degree()
    }

    fn apply_to_basis_element(&self, result : &mut FpVector, coeff : u32, input_degree : i32, input_idx : usize) {
        let p = self.prime();
        let num_gens = self.map.get_source().get_number_of_gens_in_degree(input_degree);
        let old_slice = result.get_slice();
        for i in 0 .. num_gens {
            let output = self.map.get_output(input_degree, num_gens);
            let (block_start, block_size) = self.target.block_structures.generator_to_block(input_degree, input_idx);
            result.set_slice(block_start, block_start + block_size);
            self.target.evaluate_basis_map_on_element(result, coeff, input_degree, input_idx, output);
            result.restore_slice(old_slice);
        }
    }

    fn get_lock(&self) -> MutexGuard<i32> {
        self.max_computed_degree.lock().unwrap()
    }

    fn get_max_kernel_degree(&self) -> i32 {
        0
    }

    fn set_quasi_inverse(&self, lock : &MutexGuard<i32>, degree : i32, kernel : QuasiInverse){

    }

    fn get_quasi_inverse(&self, degree : i32) -> Option<&QuasiInverse> {
        None
    }
}