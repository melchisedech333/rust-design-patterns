
/// Implementação concreta da trait Department. Note que aqui, diferente do caso
/// da Cashier (Caixa), nós fazemos o uso da função new() para instanciar nossos
/// objetos, pois nesse caso queremos demonstrar o que ocorre quando há próximos
/// elementos existentes em nosso encadeamento de handlers.

use crate::patient::Patient;
use crate::department::Department;

pub struct Doctor {
    next: Option<Box<dyn Department>>,
}

impl Doctor {
    
    // Note  que ao instanciar o objeto, passamos como parâmetro qualquer imple-
    // mentação  da trait Department. Esse parâmetro é salvo no atributo next da
    // struct que corresponde ao próprio objeto instanciado (Doctor).
    //
    // Desta maneira, na implementação padrão da função execute() do BaseHandler
    // ao  invocarmos  o next.execute(), estaremos na verdade acessando o método
    // execute()  do  objeto que foi aqui passado como parâmetro na instanciação
    // deste objeto em questão (Doctor).
    pub fn new(next: impl Department + 'static) -> Self {
        Self {
            next: Some(Box::new(next))
        }
    }
}

impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.doctor_check_up_done {
            println!("A doctor checkup is already done");
        } else {
            println!("Doctor checking a patient {}", patient.name);
            patient.doctor_check_up_done = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}


