
/// Código representante das coisas que ficam no campo da Implementação.
/// 
/// Note que utilizamos a Trait Device de modo internamente também abstrato,
/// onde iremos implementar várias outras Traits concretas para os Device's.
/// Dessa maneira, tanto no lado da Abstração quanto no lado da Implementação
/// temos liberdade para realizar implementações totalmente customizadas sem
/// fazer com que uma coisa dependa da outra necessariamente, dando assim maior
/// flexilidade para o crescimento do projeto.

mod radio;
mod tv;

pub use radio::Radio;
pub use tv::Tv;

pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn volume(&self) -> u8;
    fn set_volume(&mut self, percent: u8);
    fn channel(&self) -> u16;
    fn set_channel(&mut self, channel: u16);
    fn print_status(&self);
}


