
trait Component {
    fn operation(&self) -> String;
}

trait Decorator<T: Component>: Component {
    fn new(component: T) -> Self;
}

struct MyComponent {}

impl Component for MyComponent {
    fn operation(&self) -> String {
        "My Component".to_string()
    }
}

struct MyOtherComponent {}

impl Component for MyOtherComponent {
    fn operation(&self) -> String {
        "My Other Component".to_string()
    }
}

struct MyDecorator<T: Component> {
    component: T,
}

impl<T: Component> Decorator<T> for MyDecorator<T> {
    fn new(component: T) -> Self {
        MyDecorator { component }
    }
}

impl<T: Component> Component for MyDecorator<T> {
    fn operation(&self) -> String {
        format!("MyDecorator: {}", self.component.operation())
    }
}

fn main() {
    let my_component_decorator = MyDecorator::new(MyComponent {});
    let my_other_component_decorator = MyDecorator::new(MyOtherComponent {});

    println!("{}", my_component_decorator.operation());
    println!("{}", my_other_component_decorator.operation());
}


