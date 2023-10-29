
/// Como  essa  implementação  concreta  da trait Department será a última a ser
/// chamada,  a  construção  do objeto não será feita com a new(), como é o caso
/// das  outras implementações concretas (como a Doctor), pois desta maneira po-
/// deremos  simular o que ocorre quando se chega no final do encadeamento. Pois
/// no método execute() da implementação padrão verificamos o Option<T> do nosso
/// atributo next, e devido a não inicializarmos ele, o mesmo retornará um None.

use crate::patient::Patient;
use crate::department::Department;

#[derive(Default)]
pub struct Cashier {
    next: Option<Box<dyn Department>>,
}

impl Department for Cashier {

    // Controle das ações próprias do handler em questão.
    fn handle(&mut self, patient: &mut Patient) {
        if patient.payment_done {
            println!("Payment done");
        } else {
            println!("Cashier getting money from a patient {}", patient.name);
            patient.payment_done = true;
        }
    }

    // Retorna o que há armazenado em nosso atributo next.
    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}


