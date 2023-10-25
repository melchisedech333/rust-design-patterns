
/// Implementação da interface utilizada normalmente por algum cliente, ou seja, 
/// do código comum que será utilizado pela aplicação.

use crate::target::Target;

pub struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "Ordinary request.".into()
    }
}


