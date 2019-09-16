use crate::algebra::AlgebraAny;
use crate::module::Module;
use crate::module::homomorphism::{ModuleHomomorphism, ZeroHomomorphismT};
use crate::chain_complex::{AugmentedChainComplex, ChainComplex};
use std::sync::Arc;

pub struct FiniteChainComplex<M, F>
where M : Module,
      F : ModuleHomomorphism<Source=M, Target=M> {
    pub modules : Vec<Arc<M>>,
    pub zero_module : Arc<M>,
    pub differentials : Vec<Arc<F>>
}

impl<M, F> FiniteChainComplex<M, F>
where M : Module,
      F : ModuleHomomorphism<Source=M, Target=M> + ZeroHomomorphismT<M, M> {

    pub fn pop(&mut self) {
        if self.modules.len() == 0 {
            return;
        }
        self.modules.pop();
        if self.modules.len() == 0 {
            self.differentials.drain(0..self.differentials.len() - 2);
        } else {
            let len = self.differentials.len();
            self.differentials.remove(len - 2);
            self.differentials[len - 3] = Arc::new(F::zero_homomorphism(self.zero_module(), Arc::clone(&self.modules[self.modules.len() - 1]), 0));
        }
    }
}

impl<M, F> ChainComplex for FiniteChainComplex<M, F>
where M : Module,
      F : ModuleHomomorphism<Source=M, Target=M> {
    type Module = M;
    type Homomorphism = F;

    fn algebra(&self) -> Arc<AlgebraAny> {
        self.zero_module.algebra()
    }
    fn min_degree(&self) -> i32 {
        self.zero_module.min_degree()
    }

    fn zero_module(&self) -> Arc<Self::Module> {
        Arc::clone(&self.zero_module)
    }

    fn module(&self, s : u32) -> Arc<Self::Module> {
        let s = s as usize;
        if s >= self.modules.len() {
            self.zero_module()
        } else {
            Arc::clone(&self.modules[s])
        }
    }

    fn differential(&self, s : u32) -> Arc<Self::Homomorphism> {
        let s = s as usize;
        let s = std::cmp::min(s, self.differentials.len() - 1); // The last entry is the zero homomorphism
        Arc::clone(&self.differentials[s])
    }

    fn compute_through_bidegree(&self, homological_degree : u32, internal_degree : i32) {}

    fn set_homology_basis(&self, homological_degree : u32, internal_degree : i32, homology_basis : Vec<usize>) { unimplemented!() }
    fn homology_basis(&self, homological_degree : u32, internal_degree : i32) -> &Vec<usize> { unimplemented!() }
    fn max_homology_degree(&self, homological_degree : u32) -> i32 { std::i32::MAX }
}

pub struct FiniteAugmentedChainComplex<M, F1, F2, CC>
where M : Module,
      CC : ChainComplex,
      F1 : ModuleHomomorphism<Source=M, Target=M>,
      F2 : ModuleHomomorphism<Source=M, Target=CC::Module> {
    pub modules : Vec<Arc<M>>,
    pub zero_module : Arc<M>,
    pub differentials : Vec<Arc<F1>>,
    pub target_cc : Arc<CC>,
    pub chain_maps : Vec<Arc<F2>>
}

impl<M, F1, F2, CC> ChainComplex for FiniteAugmentedChainComplex<M, F1, F2, CC>
where M : Module,
      CC : ChainComplex,
      F1 : ModuleHomomorphism<Source=M, Target=M>,
      F2 : ModuleHomomorphism<Source=M, Target=CC::Module> {
    type Module = M;
    type Homomorphism = F1;

    fn algebra(&self) -> Arc<AlgebraAny> {
        self.zero_module.algebra()
    }
    fn min_degree(&self) -> i32 {
        self.zero_module.min_degree()
    }

    fn zero_module(&self) -> Arc<Self::Module> {
        Arc::clone(&self.zero_module)
    }

    fn module(&self, s : u32) -> Arc<Self::Module> {
        let s = s as usize;
        if s >= self.modules.len() {
            self.zero_module()
        } else {
            Arc::clone(&self.modules[s])
        }
    }

    fn differential(&self, s : u32) -> Arc<Self::Homomorphism> {
        let s = s as usize;
        let s = std::cmp::min(s, self.differentials.len() - 1); // The last entry is the zero homomorphism
        Arc::clone(&self.differentials[s])
    }

    fn compute_through_bidegree(&self, homological_degree : u32, internal_degree : i32) {}

    fn set_homology_basis(&self, homological_degree : u32, internal_degree : i32, homology_basis : Vec<usize>) { unimplemented!() }
    fn homology_basis(&self, homological_degree : u32, internal_degree : i32) -> &Vec<usize> { unimplemented!() }
    fn max_homology_degree(&self, homological_degree : u32) -> i32 { std::i32::MAX }
}

impl<M, F1, F2, CC> AugmentedChainComplex for FiniteAugmentedChainComplex<M, F1, F2, CC>
where M : Module,
      CC : ChainComplex,
      F1 : ModuleHomomorphism<Source=M, Target=M>,
      F2 : ModuleHomomorphism<Source=M, Target=CC::Module> {
    type TargetComplex = CC;
    type ChainMap = F2;

    fn target(&self) -> Arc<Self::TargetComplex> {
        Arc::clone(&self.target_cc)
    }

    /// This currently crashes if `s` is greater than the s degree of the class this came from.
    fn chain_map(&self, s: u32) -> Arc<Self::ChainMap> {
        Arc::clone(&self.chain_maps[s as usize])
    }
}

impl<M, F1, F2, CC> From<FiniteAugmentedChainComplex<M, F1, F2, CC>> for FiniteChainComplex<M, F1>
where M : Module,
      CC : ChainComplex,
      F1 : ModuleHomomorphism<Source=M, Target=M>,
      F2 : ModuleHomomorphism<Source=M, Target=CC::Module> {

    fn from(c : FiniteAugmentedChainComplex<M, F1, F2, CC>) -> FiniteChainComplex<M, F1> {
        FiniteChainComplex {
            modules : c.modules.clone(),
            zero_module : Arc::clone(&c.zero_module),
            differentials : c.differentials.clone()
        }
    }
}
