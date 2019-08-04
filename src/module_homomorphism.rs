use std::sync::{Mutex, MutexGuard};

use crate::fp_vector::{FpVector, FpVectorT};
use crate::matrix::{Matrix, Subspace, QuasiInverse};
use crate::module::Module;

pub trait ModuleHomomorphism {
    fn get_source(&self) -> &Module;
    fn get_target(&self) -> &Module;

    fn get_min_degree(&self) -> i32 {
        self.get_source().get_min_degree()
    }

    fn apply_to_basis_element(&self, result : &mut FpVector, coeff : u32, input_degree : i32, input_idx : usize);
    
    fn get_prime(&self) -> u32 {
        self.get_source().get_prime()
    }

    fn get_lock(&self) -> MutexGuard<i32>;

    fn set_kernel(&self, lock : &MutexGuard<i32>, degree : i32, kernel : Subspace);
    fn get_kernel(&self, degree : i32) -> Option<&Subspace>;

    fn set_quasi_inverse(&self, lock : &MutexGuard<i32>, degree : i32, kernel : QuasiInverse);    
    fn get_quasi_inverse(&self, degree : i32) -> Option<&QuasiInverse>;

    fn get_image(&self, degree : i32) -> Option<&Subspace> {
        let option_quasi_inverse = self.get_quasi_inverse(degree);
        return option_quasi_inverse.map(|quasi_inverse| &quasi_inverse.image );    
    }
    // fn get_image_pivots(&self, degree : i32) -> Option<&Vec<isize>> {
    //     let image = self.get_image(degree);
    //     return image.map(|subspace| &subspace.column_to_pivot_row );
    // }
    
    fn get_matrix(&self, matrix : &mut Matrix, degree : i32, start_row : usize, start_column : usize) -> (usize, usize) {
        let source_dimension = self.get_source().get_dimension(degree);
        let target_dimension = self.get_target().get_dimension(degree);
        assert!(source_dimension <= matrix.get_rows());
        assert!(target_dimension <= matrix.get_columns());
        for input_idx in 0 .. source_dimension {
            // Writing into slice.
            // Can we take ownership from matrix and then put back? 
            // If source is smaller than target, just allow add to ignore rest of input would work here.
            let output_vector = &mut matrix[start_row + input_idx];
            output_vector.set_slice(start_column, start_column + target_dimension);
            self.apply_to_basis_element(output_vector, 1, degree, input_idx);
            output_vector.clear_slice();
        }
        return (start_row + source_dimension, start_column + target_dimension);
    }    
}

pub struct ZeroHomomorphism<'a, 'b> {
    source : &'a Module,
    target : &'b Module,
    max_degree : Mutex<i32>
}

impl<'a, 'b> ZeroHomomorphism<'a, 'b> {
    pub fn new(source : &'a Module, target : &'b Module) -> Self {
        let max_degree =  Mutex::new(source.get_min_degree() - 1);
        ZeroHomomorphism {
            source,
            target,
            max_degree
        }
    }
}

impl ModuleHomomorphism for ZeroHomomorphism<'_, '_> {
    fn get_source(&self) -> &Module {
        return self.source;
    }

    fn get_target(&self) -> &Module {
        return self.target;
    }

    fn apply_to_basis_element(&self, _result : &mut FpVector, _coeff : u32, _input_degree : i32, _input_idx : usize){}

    fn get_lock(&self) -> MutexGuard<i32> {
        self.max_degree.lock().unwrap()
    }

    fn set_kernel(&self, lock : &MutexGuard<i32>, degree : i32, kernel : Subspace){}
    fn get_kernel(&self, degree : i32) -> Option<&Subspace> { None }

    fn set_quasi_inverse(&self, lock : &MutexGuard<i32>, degree : i32, kernel : QuasiInverse){}    
    fn get_quasi_inverse(&self, degree : i32) -> Option<&QuasiInverse>{ None }
}
