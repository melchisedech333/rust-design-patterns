
use crate::device::Device;
use super::{HasMutableDevice, Remote};

pub struct AdvancedRemote<D: Device> {
    device: D,
}

impl<D: Device> AdvancedRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }

    // Implementa método especial, para expressar a personalização realizada
    // pela Trait em questão. Observe também que no lado da implementação (Device)
    // será chamado um método já existente, mas onde aqui ele é utilizado de maneira
    // única set_volume(0). Mas também nada impede que exista no campo da implementação
    // funcionalidades que seriam utilizadas apenas por Remote's personalizados.
    pub fn mute(&mut self) {
        println!("Remote: mute");
        self.device.set_volume(0);
    }
}

impl<D: Device> HasMutableDevice<D> for AdvancedRemote<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

impl<D: Device> Remote<D> for AdvancedRemote<D> {}


