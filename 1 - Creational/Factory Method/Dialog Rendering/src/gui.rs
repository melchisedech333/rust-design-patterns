
// Interface do produto que sempre será retornada pelo método fábrica.
// Observe que é centralizado as ações do render e on_click.
// Onde o render será posteriormente chamado pela abstração base (Dialog).
pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

// Abstração base que centraliza tudo, também chamada de classe Creator em tutoriais.
pub trait Dialog {

    // Método fábrica, ele não possui uma implementação, pois deve ser implementado
    // pelos tipos de produtos existentes. No caso os tipos de Button.
    // Retorna um Box, onde nele vai qualquer tipo que implemente a Trait Button.
    fn create_button(&self) -> Box<dyn Button>;

    //
    // Métodos padrões.
    //

    // Renderiza o componente de acordo com a configuração (Windows ou HTML).
    // Criando uma nova instância do produto (Button) e invocando seu método render().
    fn render(&self) {
        let button = self.create_button();
        button.render();
    }

    fn refresh(&self) {
        println!("Dialog - Refresh");
    }
}


