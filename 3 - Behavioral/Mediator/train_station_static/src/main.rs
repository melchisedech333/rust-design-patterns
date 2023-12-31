
/// Código cliente fazendo uso da interface da estação e exemplificando o uso
/// do padrão. Essa é uma abordagem estática do conceito do Mediator, também
/// conhecida como "Mediator with Top-Down Ownership".

mod train_station;
mod trains;

use train_station::TrainStation;
use trains::{FreightTrain, PassengerTrain};

fn main() {
    let train1 = PassengerTrain::new("Train 1");
    let train2 = FreightTrain::new("Train 2");

    // Station has `accept` and `depart` methods,
    // but it also implements `Mediator`.
    let mut station = TrainStation::default();

    // Station is taking ownership of the trains.
    station.accept(train1.clone());
    station.accept(train1); // Para testar a estação, não permitindo alocar duas
                            // vezes o mesmo trem.
    station.accept(train2);

    // `train1` and `train2` have been moved inside,
    // but we can use train names to depart them.
    station.depart("Train 1");
    station.depart("Train 2");

    // Para testar a estação, não permitindo que um trem seja removido mais de
    // uma vez da estação, quando ele já foi removido ou não existe mais nela
    // por algum motivo.
    station.depart("Train 2");
    station.depart("Train 3");
}


