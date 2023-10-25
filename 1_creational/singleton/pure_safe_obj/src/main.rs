
// Classe para controle dos dados.
pub trait SingletonDataAbstract {
    fn get_value(&mut self) -> i32;
    fn change_add(&mut self);
}

pub struct SingletonData {
    value: i32
}

impl SingletonData {
    pub fn new() -> Self {
        Self {
            value: 0
        }
    }
}

impl SingletonDataAbstract for SingletonData {
    fn get_value(&mut self) -> i32 {
        self.value
    }

    fn change_add(&mut self) {
        self.value += 1;
    }
}

fn main() {

    let mut singleton_data = SingletonData::new();

    println!("Value: {}", singleton_data.get_value());
    singleton_data.change_add();
    println!("Value: {}", singleton_data.get_value());
    singleton_data.change_add();
    println!("Value: {}", singleton_data.get_value());

    // Exemplo de uso encadeado em programa comum.
    example_1(&mut singleton_data);
}

fn example_1(singleton_data: &mut impl SingletonDataAbstract) {
    singleton_data.change_add();
    println!("Value (example 1): {}", singleton_data.get_value());

    example_2(singleton_data);

    singleton_data.change_add();
    println!("Value (example 1): {}", singleton_data.get_value());
}

fn example_2(singleton_data: &mut impl SingletonDataAbstract) {
    singleton_data.change_add();
    println!("Value (example 2): {}", singleton_data.get_value());

    example_3(singleton_data);
}

fn example_3(singleton_data: &mut impl SingletonDataAbstract) {
    singleton_data.change_add();
    println!("Value (example 3): {}", singleton_data.get_value());
}


