use std::sync::{ Mutex, MutexGuard };
use crate::once::OnceBiVec;
use bivec::BiVec;
use crate::fp_vector::{FpVector, FpVectorT};
use crate::algebra::{Algebra, AlgebraAny};
use crate::module::Module;
use std::rc::Rc;

#[derive(Debug)]
pub struct OperationGeneratorPair {
    pub operation_degree : i32,
    pub operation_index : usize,
    pub generator_degree : i32,
    pub generator_index : usize
}

pub struct FreeModuleTableEntry {
    pub num_gens : usize,
    pub basis_element_to_opgen : Vec<OperationGeneratorPair>,
    pub generator_to_index : BiVec<Vec<usize>>,
}

pub struct FreeModule {
    pub algebra : Rc<AlgebraAny>,
    pub name : String,
    pub min_degree : i32,
    pub max_degree : Mutex<i32>,
    gen_names : OnceBiVec<Vec<String>>,
    pub table : OnceBiVec<FreeModuleTableEntry>
}

impl Module for FreeModule {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_algebra(&self) -> Rc<AlgebraAny> {
        Rc::clone(&self.algebra)
    }

    fn get_min_degree(&self) -> i32 {
        self.min_degree
    }

    fn get_dimension(&self, degree : i32) -> usize {
        // println!("Get dimension of {} in degree {}", self.name, degree);
        if degree < self.min_degree {
            return 0;
        }
        assert!(degree < self.table.len(), format!("Free Module {} not computed through degree {}", self.get_name(), degree));
        return self.table[degree].basis_element_to_opgen.len();
    }

    fn basis_element_to_string(&self, degree : i32, idx : usize) -> String {
        let opgen = self.index_to_op_gen(degree, idx);
        let mut op_str = self.algebra.basis_element_to_string(opgen.operation_degree, opgen.operation_index);
        if &*op_str == "1" {
            op_str = "".to_string();
        } else {
            op_str.push(' ');
        }
        return format!("{}{}", op_str, self.gen_names[opgen.generator_degree][opgen.generator_index]);
    }

    fn act_on_basis(&self, result : &mut FpVector, coeff : u32, op_degree : i32, op_index : usize, mod_degree : i32, mod_index : usize){
        assert!(op_index < self.get_algebra().get_dimension(op_degree, mod_degree));
        assert!(self.get_dimension(op_degree + mod_degree) <= result.get_dimension());
        let operation_generator = self.index_to_op_gen(mod_degree, mod_index);
        let module_operation_degree = operation_generator.operation_degree;
        let module_operation_index = operation_generator.operation_index;
        let generator_degree = operation_generator.generator_degree; 
        let generator_index  = operation_generator.generator_index;


        // Now all of the output elements are going to be of the form s * x. Find where such things go in the output vector.
        let num_ops = self.get_algebra().get_dimension(module_operation_degree + op_degree, generator_degree);
        let output_block_min = self.operation_generator_to_index(module_operation_degree + op_degree, 0, generator_degree, generator_index);
        let output_block_max = output_block_min + num_ops;
        // Writing into slice (can we take ownership? make new vector with 0's outside range and add separately? is it okay?)
        let old_slice = result.get_slice();
        result.set_slice(output_block_min, output_block_max); 
        // Now we multiply s * r and write the result to the appropriate position.
        self.get_algebra().multiply_basis_elements(result, coeff, op_degree, op_index, module_operation_degree, module_operation_index, generator_degree);
        result.restore_slice(old_slice);
    }
}

impl FreeModule {
    pub fn new(algebra : Rc<AlgebraAny>, name : String, min_degree : i32) -> Self {
        Self {
            algebra,
            name,
            min_degree,
            max_degree : Mutex::new(min_degree - 1),
            gen_names : OnceBiVec::new(min_degree),
            table : OnceBiVec::new(min_degree)
        }
    }

    pub fn get_number_of_gens_in_degree(&self, degree : i32) -> usize {
        if degree < self.min_degree {
            return 0;
        }
        return self.table[degree].num_gens;
    }

    pub fn construct_table(&self, degree : i32) -> (MutexGuard<i32>, FreeModuleTableEntry) {
        assert!(degree >= self.min_degree);
        let lock = self.max_degree.lock().unwrap();
        assert!(degree == *lock + 1);
        let mut basis_element_to_opgen : Vec<OperationGeneratorPair> = Vec::new();
        let mut generator_to_index : BiVec<Vec<usize>> = BiVec::new(self.min_degree);
        // gen_to_idx goes gen_degree => gen_idx => start of block.
        // so gen_to_idx_size should be (number of possible degrees + 1) * sizeof(uint*) + number of gens * sizeof(uint).
        // The other part of the table goes idx => opgen
        // The size should be (number of basis elements in current degree) * sizeof(FreeModuleOperationGeneratorPair)
        // A basis element in degree n comes from a generator in degree i paired with an operation in degree n - i.
        let mut offset = 0;
        for gen_deg in self.min_degree .. degree {
            let num_gens = self.get_number_of_gens_in_degree(gen_deg);
            let mut gentoidx_degree : Vec<usize> = Vec::with_capacity(num_gens);
            let op_deg = degree - gen_deg;
            let num_ops = self.get_algebra().get_dimension(op_deg, gen_deg);
            for gen_idx in 0 .. num_gens {
                gentoidx_degree.push(offset);
                for op_idx in 0 .. num_ops {
                    basis_element_to_opgen.push(OperationGeneratorPair {
                        generator_degree : gen_deg,
                        generator_index : gen_idx,
                        operation_degree : op_deg,
                        operation_index : op_idx
                    })
                }
                offset += num_ops;
            }
            generator_to_index.push(gentoidx_degree);
        }
        return (lock,
            FreeModuleTableEntry {
                num_gens : 0,
                basis_element_to_opgen,
                generator_to_index
            }
        );
    }

    pub fn get_dimension_with_table(table : &FreeModuleTableEntry) -> usize {
        table.basis_element_to_opgen.len()
    }

    pub fn add_generators(&self, degree : i32, mut lock : MutexGuard<i32>, mut table : FreeModuleTableEntry, num_gens : usize, names : Option<Vec<String>>){
        assert!(degree >= self.min_degree);
        assert!(degree == *lock + 1);
        let mut gen_names;
        if let Some(names_vec) = names {
            gen_names = names_vec;
        } else {
            gen_names = Vec::with_capacity(num_gens);
            for i in 0 .. num_gens {
                gen_names.push(format!("x_{{{},{}}}", degree, i));
            }
        }
        self.gen_names.push(gen_names);
        Self::add_generators_to_table(degree, &mut table, num_gens);
        self.table.push(table);
        *lock += 1;
    }

    fn add_generators_to_table(degree : i32, table : &mut FreeModuleTableEntry, num_gens : usize){
        // let old_num_gens = table.num_gens;
        // let new_num_gens = old_num_gens + new_generators;
        table.num_gens = num_gens;
        let old_dimension = table.basis_element_to_opgen.len();
        let mut start_of_block = old_dimension;
        let mut gen_to_idx = Vec::with_capacity(num_gens);
        for gen_idx in 0 .. num_gens {
            table.basis_element_to_opgen.push(OperationGeneratorPair {
                generator_degree : degree,
                generator_index : gen_idx,
                operation_degree : 0,
                operation_index : 0
            });
            gen_to_idx.push(start_of_block);
            start_of_block += 1;
        }
        table.generator_to_index.push(gen_to_idx);
    }

    pub fn operation_generator_to_index(&self, op_deg : i32, op_idx : usize, gen_deg : i32, gen_idx : usize) -> usize {
        assert!(op_deg >= 0);
        assert!(gen_deg >= self.min_degree);
        return self.table[op_deg + gen_deg].generator_to_index[gen_deg][gen_idx] + op_idx;
    }

    pub fn operation_generator_pair_to_idx(&self, op_gen : &OperationGeneratorPair) -> usize {
        self.operation_generator_to_index(
            op_gen.operation_degree,
            op_gen.operation_index,
            op_gen.generator_degree,
            op_gen.generator_index
        )
    }

    pub fn index_to_op_gen(&self, degree : i32, index : usize) -> &OperationGeneratorPair {
        assert!(degree >= self.min_degree);
        return &self.table[degree].basis_element_to_opgen[index];
    }

    pub fn add_generators_immediate(&self, degree : i32, num_gens : usize, gen_names : Option<Vec<String>>){
        let (lock, table) = self.construct_table(degree);
        self.add_generators(degree, lock, table, num_gens, gen_names);
    }

    pub fn extend_by_zero(&self, degree : i32){
        let old_max_degree = { *self.max_degree.lock().unwrap() };
        for i in old_max_degree + 1 ..= degree {
            self.add_generators_immediate(i, 0, None)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    use crate::adem_algebra::AdemAlgebra;

    #[test]
    fn test_free_mod(){
        let p = 2;
        let A = Rc::new(AlgebraAny::from(AdemAlgebra::new(p, p != 2, false)));
        A.compute_basis(10);
        let M = FreeModule::new(Rc::clone(&A), "".to_string(), 0);
        let (lock, table) = M.construct_table(0);
        M.add_generators(0, lock, table, 1, None);
        let (lock, table) = M.construct_table(1);
        M.add_generators(1, lock, table, 1, None);
        for i in 2..10{
            let (lock, table) = M.construct_table(i);
            M.add_generators(i, lock, table, 0, None);
        }
        let op_deg = 2;
        let op_idx = 0;
        let input_deg = 4;
        let input_idx = 0;
        let output_deg = op_deg + input_deg;
        let output_dim = M.get_dimension(output_deg);
        for i in 0..9 {
            assert_eq!(M.get_dimension(i), A.get_dimension(i,0) + A.get_dimension(i-1,1));
        }

        for (gen_deg, gen_idx) in &[(0,0), (1,0)]{
            let idx = M.operation_generator_to_index(output_deg - *gen_deg, 0, *gen_deg, *gen_idx);
            println!("index : {}", idx);
        }
        let mut result = FpVector::new(p, output_dim);
        // M.act_on_basis(&mut result, 1, op_deg, op_idx, input_deg, input_idx);
        M.act_on_basis(&mut result, 1, 5, 0, 1, 0);
        println!("{}", result);
        println!("result : {}", M.element_to_string(output_deg, &result));
        result.set_to_zero();
        M.act_on_basis(&mut result, 1, 5, 0, 1, 1);
        println!("{}", result);
        println!("result : {}", M.element_to_string(output_deg, &result));        
        println!("1, 0 : {}", M.basis_element_to_string(1,0));
        println!("1, 1 : {}", M.basis_element_to_string(1,1));
    }

}


// uint FreeModule_element_toJSONString(char *result, FreeModule *this, int degree, Vector *element);
