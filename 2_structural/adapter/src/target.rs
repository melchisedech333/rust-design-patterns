
// Interface utilizada normalmente por algum cliente.

pub trait Target {
    fn request(&self) -> String;
}


