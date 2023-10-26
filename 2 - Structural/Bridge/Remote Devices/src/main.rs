
/// Código cliente fazendo uso das coisas referentes aos campos de Abstração e Implementação.

mod device;
mod remotes;

use device::{Device, Radio, Tv};
use remotes::{AdvancedRemote, BasicRemote, HasMutableDevice, Remote};

fn main() {

    // Note que utilizamos o método default() para criar uma nova instância do objeto.
    test_device(Tv::default());
    test_device(Radio::default());
}

// Note que passamos a Trait Device, mas também especificamos a Clone, pois especificamos
// a derive Clone lá no código da Implementação.
fn test_device(device: impl Device + Clone) {

    println!("Tests with basic remote.");
    let mut basic_remote = BasicRemote::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Tests with advanced remote.");
    let mut advanced_remote = AdvancedRemote::new(device);
    advanced_remote.power();
    advanced_remote.mute(); // Chama método especial definido em uma implementação concreta de algo do
                            // campo da Abstração. Demonstrando a flexibilidade que possuímos para modificar
                            // as coisas referente ao campo da Abstração, sem que isso afete as coisas do
                            // campo da Implementação.
    advanced_remote.device().print_status();
}


