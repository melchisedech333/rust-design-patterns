
/// Código do adaptador, note que ele implementa a Trait cliente Target, mas
/// internamente realiza a adaptação no método request().

use crate::{specific_target::SpecificTarget, target::Target};

/// Converts spec_target's specific interface to a compatible `Target` output.
pub struct TargetAdapter {
    spec_target: SpecificTarget,
}

/// Recebe como parametro no contrutor a classe específica que será adaptada.
impl TargetAdapter {
    pub fn new(spec_target: SpecificTarget) -> Self {
        Self { spec_target }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        // Here's the "adaptation" of a specific output to a compatible output.
        self.spec_target.specific_request().chars().rev().collect()
    }
}


