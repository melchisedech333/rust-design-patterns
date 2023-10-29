
/// Implementação de um cliente fazendo o uso do encadeamento dos handlers.

mod patient;
mod department;
mod cashier;
mod doctor;
mod medical;
mod reception;

use patient::Patient;
use department::Department;
use cashier::Cashier;
use doctor::Doctor;
use medical::Medical;
use reception::Reception;

fn main() {

    // Note  que aqui vamos criando a ordem de modo inverso, pois quando ela for
    // chamada,  será executado primeiro Reception, depois Doctor e assim suces-
    // sivamente até chegar em Cashier.
    let cashier       = Cashier::default();     // Item 'next' conterá None.
    let medical       = Medical::new(cashier);  // Ao iniciar com new(), o item
    let doctor        = Doctor::new(medical);   // 'next' conterá a instância
    let mut reception = Reception::new(doctor); // passada como parâmetro.

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default() // Copia o restante da struct para cá.
    };

    // A  recepção  atende o paciente, depois passa para o doutor, depois para o
    // médico, e por fim o paciente passa para o Cashier.
    //
    // Reception -> Doctor -> Medical -> Cashier.
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    // Ao realizar a chamada novamente, passando como parametro o mesmo paciente
    // os  handlers  vão  realizar as verificações, e como as checagens já foram
    // realizadas  na  chamada  anterior,  a cada handler acessado é informado o
    // status atual das coisas.
    reception.execute(&mut patient);
}


