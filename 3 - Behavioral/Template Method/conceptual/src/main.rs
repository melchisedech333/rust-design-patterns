
/// Vale citar que o Template Method é bem semelhante ao Strategy no sentido de
/// que ambos conseguem isolar o comportamento específicos de parte de um 
/// grande algoritmo. O Strategy faz isso de modo mais dinâmico devido ao uso de
/// composição, enquanto que o Template Method faz isso de modo mais estático
/// devido ao uso de herança.

// Classe base abstrata do Template Method.
trait TemplateMethod {

    // Método de construção/template, note que a ordem das ações são variadas,
    // isso é para simular uma lógica específica de algum algoritmo.
    fn template_method(&self) {
        self.base_operation1();
        self.required_operations1();
        self.base_operation2();
        self.hook1();
        self.required_operations2();
        self.base_operation3();
        self.hook2();
    }

    // Métodos padrões.
    fn base_operation1(&self) {
        println!("TemplateMethod says: I am doing the bulk of the work");
    }

    fn base_operation2(&self) {
        println!("TemplateMethod says: But I let subclasses override some operations");
    }

    fn base_operation3(&self) {
        println!("TemplateMethod says: But I am doing the bulk of the work anyway");
    }

    // Métodos para serem implementados pelas implementações concretas da 
    // classe base da Template Method.
    fn hook1(&self) {}
    fn hook2(&self) {}

    fn required_operations1(&self);
    fn required_operations2(&self);
}

// Exemplos de implementações concretas da classe base Template Method.
struct ConcreteStruct1;

impl TemplateMethod for ConcreteStruct1 {
    fn required_operations1(&self) {
        println!("ConcreteStruct1 says: Implemented Operation1")
    }

    fn required_operations2(&self) {
        println!("ConcreteStruct1 says: Implemented Operation2")
    }
}

struct ConcreteStruct2;

impl TemplateMethod for ConcreteStruct2 {
    fn required_operations1(&self) {
        println!("ConcreteStruct2 says: Implemented Operation1")
    }

    fn required_operations2(&self) {
        println!("ConcreteStruct2 says: Implemented Operation2")
    }
}

// Exemplo de código cliente que executa somente o método padrão 
// template_method()
fn client_code(concrete: impl TemplateMethod) {
    concrete.template_method()
}

// Lógica da aplicação que faz o uso do cliente.
fn main() {
    println!("Same client code can work with different concrete implementations:");
    client_code(ConcreteStruct1);
    println!();

    println!("Same client code can work with different concrete implementations:");
    client_code(ConcreteStruct2);
}


