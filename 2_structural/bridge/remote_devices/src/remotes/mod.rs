
/// O conceito do Bridge separa o programa em dois grandes campos/áreas, sendo eles a
/// Abstração e a Implementação (bem como é aceito outras categorias, como frontend e backend, etc).
/// Esse código corresponde a interface que controle a Abstração.

/// Necessário para realizar as importações.
mod advanced;
mod basic;

pub use advanced::AdvancedRemote;
pub use basic::BasicRemote;

use crate::device::Device;

/// Declara uma Trait com um método que sempre retorna alguma implementação da Trait Device.
/// Note que é utilizado um tipo genérico D com "limite de Trait" para aceitar apenas alguma
/// implementação da Trait Device.
pub trait HasMutableDevice<D: Device> {
    fn device(&mut self) -> &mut D;
}

/// Realiza uma herança. Note que é também utilizado limite de Trait para os tipos.
/// Esta Trait funciona como o código do campo da Abstração do conceito, servindo também
/// como um código abstrato para ser posteriormente implementado por implementações concretas,
/// tais como BasicRemote.
/// 
/// Note que da Trait da herança é herdado também o método abstrato e sem implementação concreta device().
/// Ele será implementado pelas implementações concretas de Remote.
pub trait Remote<D: Device>: HasMutableDevice<D> {
    fn power(&mut self) {
        println!("Remote: power toggle");
        if self.device().is_enabled() {
            self.device().disable();
        } else {
            self.device().enable();
        }
    }

    fn volume_down(&mut self) {
        println!("Remote: volume down");
        let volume = self.device().volume();
        self.device().set_volume(volume - 10);
    }

    fn volume_up(&mut self) {
        println!("Remote: volume up");
        let volume = self.device().volume();
        self.device().set_volume(volume + 10);
    }

    fn channel_down(&mut self) {
        println!("Remote: channel down");
        let channel = self.device().channel();
        self.device().set_channel(channel - 1);
    }

    fn channel_up(&mut self) {
        println!("Remote: channel up");
        let channel = self.device().channel();
        self.device().set_channel(channel + 1);
    }
}


