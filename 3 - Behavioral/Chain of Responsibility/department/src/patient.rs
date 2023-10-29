
/// Este código trata-se da estrutura de dados que passará pelos diversos níveis
/// de handlers implementados. No caso pelos vários niveis de handlers será pas-
/// sado a instancia de algum objeto, no caso o que será passado é uma instância
/// dessa struct.
/// 
/// Esse  tipo de objeto, struct, etc, também é chamado de Request, pois ela re-
/// presenta  a  requisição sendo processada pelo encadeamento dos Handlers con-
/// cretos implementados.

#[derive(Default)]
pub struct Patient {
    pub name: String,
    pub registration_done: bool,
    pub doctor_check_up_done: bool,
    pub medicine_done: bool,
    pub payment_done: bool,
}


