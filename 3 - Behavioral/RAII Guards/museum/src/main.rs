
use std::cell::RefCell;
use std::ops::Drop;

/// Trait controladora do recurso. Todos os recursos são gerados através da
/// mesma, mais especificamente do método get_ticket(). Note que utilizamos
/// RefCell para possuir múltiplas referências mutáveis a uma mesma região da
/// memória, isso se dá devido a nosso recurso (implementado pela trait Ticket)
/// receber como parâmetro o nosso self, ou seja, uma referência a instância do
/// objeto instanciado de Museum, onde internamente em nossa trait Ticket
/// implementados a trait Drop, que por sua vez irá acessaro nosso recurso
/// criado com RefCell, ou seja, múltiplas instâncias de Ticket vão possuir
/// referências mutáveis ao mesmo item, que no caso é nosso atributo chamado
/// 'remaining_tickets'. Por fim vale citar que isso só se aplica em situação
/// single-threaded, para multi-threading é necessário utilizar outros métodos.
pub struct Museum {

    // Cria uma região na memória que pode possuir múltiplas referências 
    // mutáveis, e especifica que o tipo de dado ali armazenado é u32.
    remaining_tickets: RefCell<u32>,
}

impl Museum {
    pub fn new() -> Self {
        Museum {
            // Realiza a criação da referência e armazena o valor inteiro 20.
            remaining_tickets: RefCell::new(20),
        }
    }

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {

        // Retorna uma referência mutável, e modificamos os dados.
        let mut data = self.remaining_tickets.borrow_mut();
        if *data > 0 {
            *data -= 1;
            Some(Ticket { museum: self })
        } else {
            None
        }
    }
}

// Recurso que desejo controlar.
pub struct Ticket<'a> {
    museum: &'a Museum,
}

impl Ticket<'_> {
    pub fn admire_exhibits(&self) {
        println!("How amazing!");
    }
}

// Implementação da trait Drop para realizar a liberação do recurso 
// automaticamente ao objeto não possuir mais referências.
impl Drop for Ticket<'_> {
    fn drop(&mut self) {
        let mut data = self.museum.remaining_tickets.borrow_mut();
        *data += 1;
    }
}

// Código cliente para demonstração.
fn main() {
    
    // Instancia um museu.
    let museum = Museum::new();

    // Gera 20 tickets e armazena-os num vetor.
    let mut tickets: Vec<Ticket> = (0..20)
        .map(|_| museum.get_ticket().unwrap())
        .collect();
    
    println!("Total tickets: {}", tickets.len());

    // Tenta gerar mais 1 ticket, note que aqui é retornado um None, pois
    // o recurso está limitado em apenas 20 itens.
    match museum.get_ticket() {
        Some(_) => println!("New ticket!"),
        None => println!("No ticket found."),
    };

    // Libera um dos tickets armazenados no vetor de tickets, e solicita um
    // novo ticket. Note que desta vez ele é alocado.
    tickets.pop();
    println!("Total tickets: {}", tickets.len());
    tickets.push(museum.get_ticket().unwrap());
    println!("Total tickets: {}", tickets.len());

    // Segue mais um exemplo para demonstrar o conceito. Note que foi liberado
    // Um recurso do array de tickets, e em seguida verificado se existe
    // recursos disponíveis (e diz que possui 1). Em seguida criamos uma 
    // variável chamada 'tick' e armazenamos mais um recurso, ao verificar os
    // recursos disponíveis é dito que eles não estão mais disponíveis. O que é
    // importante ter em mente aqui, é que o recurso fica alocado até que todas
    // as referências ao mesmo sejam destruídas, ou de alguma maneira o recurso
    // seja destruído, como é o caso com o uso da função drop() que vem logo em
    // seguida.
    tickets.pop();
    println!("Resources found 1: {}", *museum.remaining_tickets.borrow());

    let tick :Ticket = museum.get_ticket().unwrap();
    println!("Resources found 2: {}", *museum.remaining_tickets.borrow());

    drop(tick);
    println!("Resources found 3: {}", *museum.remaining_tickets.borrow());

    // Mais um exemplo para demonstrar a alocação e liberação do recurso. Note
    // que ao sair do bloco de código, o recurso é liberado automaticamente.   
    {
        let tick :Ticket = museum.get_ticket().unwrap();
        println!("Resources found 4: {}", *museum.remaining_tickets.borrow());   
    }

    println!("Resources found 5: {}", *museum.remaining_tickets.borrow());   

    // Aqui retornamos o total de recursos disponíveis segundo nossa RAII guard.
    // Note que o valor retornado é zero, pois ele foi decrementado. Depois,
    // quando utilizamos a função drop() e excluímos os tickets existentes no
    // vetor, acabamos que liberando o recurso, e quando vamos em seguida 
    // verificar os recursos disponíveis para uso, recebemos o valor 20. Pois
    // desta vez eles foram incrementados por nossa trait Drop, que só é
    // chamada na destruição de nossos objetos.
    drop(tickets);
    println!("Resources found 6: {}", *museum.remaining_tickets.borrow());
}


