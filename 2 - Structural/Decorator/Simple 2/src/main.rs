
// Component.
trait Component {
    fn do_something(&self);
}

struct BaseObject {
    value: usize,
}

impl Component for BaseObject {
    fn do_something(&self) {
        println!("calculate value: {}", self.value);
    }
}

// Decorator.
trait Decorator: Component {
    fn do_something_more(&self);
}

struct DecoratorObject {
    base: BaseObject,
    more_value: usize,
}

impl Component for DecoratorObject {
    fn do_something(&self) {
        self.base.do_something()
    }
}

impl Decorator for DecoratorObject {
    fn do_something_more(&self) {
        println!("calculate more: {}", self.more_value);
    }
}

// Client.
pub fn decorator() {
    let obj = BaseObject { value: 100 };
    process(&obj);

    let decorator_obj = DecoratorObject { base: obj, more_value: 999 };
    process(&decorator_obj);

    decorator_obj.do_something_more();
}

fn process(c: &dyn Component) {
    c.do_something();
}

fn main() {
    decorator();
}


