
/// Na explicação do conceito é comum encontrar um BaseHandler, que é uma imple-
/// mentação  abstrata do Handler, que será implementado pelos ConcreteHandlers.
/// Podemos  possuir  uma interface abstrata e um BaseHandler que implementa al-
/// guns  métodos  padrões,  que serão herdados (via herança) por implementações
/// concretas.  Indepente  o  modo com que se realiza esse ponto, o importante é 
/// possuir uma abstração padrão onde os handlers concretos vão utilizá-la.

use crate::patient::Patient;

pub trait Department {

    // Sendo  uma implementação padrão, contamos com dois métodos abstratos e um
    // concreto,  que  é esse execute(). Onde o mesmo é responsável por adentrar
    // no encadeamento dos handlers de maneira recursiva.
    //
    // Note  que quem irá chamar este método é o cliente, onde será passado como
    // parâmetro um Patient para ser processado.
    //
    // Note  que  a ordem de chamar a self.handle() primeiro e depois realizar a
    // verificação  do self.next() pode ser alterada. Quando isso é feito ocorre
    // da verificação ser feita em ordem inversa.
    fn execute(&mut self, patient: &mut Patient) {
        
        // O método  handle  é  uma implementação feita por alguma implementação
        // concreta  de  Department. Nela é realizada o controle próprio do han-
        // dler. Logo, por isso ele é chamado logo no começo desta função execu-
        // te().
        self.handle(patient);
        
        // Note que as ações referente ao handler em questão serão realizadas, e
        // logo  em seguida chegamos nesse código, onde verificamos se existe um
        // próximo objeto na cadeia de handlers, caso exista, invocamos a função
        // execute()  do  mesmo. Pois todas as implementações concretas da trait
        // Department  também possuem esse método execute(), e devido a isso po-
        // demos invoca-lo, realizando assim um encadeamento.
        //
        // Note  que  utilizamos o Some(T), pois o atributo next é um Option<T>.
        // Logo,  quando  não  temos  mais  objetos  referenciados na next, será 
        // retornado um None, indicando assim que se chegou ao fim do encademen-
        // to.
        if let Some(next) = &mut self.next() {
            next.execute(patient);
        }
    }

    // Implementação para processar as ações do handler em questão.
    fn handle(&mut self, patient: &mut Patient);

    // Implementação que irá sempre retornar a instância presente no atributo da
    // struct implementada (atributo next).
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;
}


