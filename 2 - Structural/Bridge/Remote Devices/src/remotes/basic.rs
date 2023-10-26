
/// Implementação concreta da interface da área de Abstração (segundo o conceito do Bridge).

/// Necessário para realizar as importações no programa.
use crate::device::Device;
use super::{HasMutableDevice, Remote};

/// Cria uma struct com um tipo D, mas especifica que esse D deve ser alguma 
/// implementação da Trait Device.
pub struct BasicRemote<D: Device> {
    device: D,  // PONTO MUITO IMPORTANTE: aqui armazenamos a instância de alguma
                // implementação da Trait Device. Para assim poder utilizá-la em
                // tempo de execução. E com isso também realizando a parte do conceito
                // do Bridge referente a COMPOSIÇÃO DO OBJETO, onde ao invés de 
                // trabalharmos com herança, trabalhamos com uma instância do objeto
                // existente internamente na classe.
}

/// Implementa a BasicRemote, mas note que logo após o 'impl' é necessário
/// definirmos o tipo de concreto de D, ou seja, informando que D corresponde
/// a Trait chamada Device, ou qualquer implementação da mesma.
impl<D: Device> BasicRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }
}

/// O mesmo procedimento ocorre aqui, ao implementar a Trait HasMutableDevice
/// para a Trait BasicRemote, antes de tudo devemos especificar após o 'impl'
/// o tipo correspondente a D, ou seja, nossa Trait Device ou qualquer 
/// implementação da mesma.
impl<D: Device> HasMutableDevice<D> for BasicRemote<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

/// Criamos essa implementação sem nada dentro, apenas para que seja herdado
/// os métodos já implementados em Remote, para dentro de BasicRemote.
/// Métodos como power(), volume_down() e outros.
impl<D: Device> Remote<D> for BasicRemote<D> {}


