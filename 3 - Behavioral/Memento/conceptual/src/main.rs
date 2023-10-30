
/// Exemplo conceitual.

/// Interface abstrata do Memento. Seguida da sua implementação concreta
/// OriginatorBackup. Note que o método restore() retorna um objeto Originator
/// com o estado 'state', que havia ficado salvo na Memento (OriginatorBackup).
trait Memento<T> {
    fn restore(self) -> T;
    fn print(&self);
}

struct OriginatorBackup {
    state: String, // Estado para guardar.
}

impl Memento<Originator> for OriginatorBackup {
    fn restore(self) -> Originator {
        Originator {
            state: self.state.parse().unwrap(),
        }
    }

    fn print(&self) {
        println!("Originator backup: '{}'", self.state);
    }
}

/// Implementação concreta de uma classe Originadora, isto é, a classe que deve
/// ser feito o snapshot/backup de seus estados. Note que quem realiza o backup
/// é a própria classe originadora a partir do método save(), fazendo assim com
/// que somente a originadora possa realizar backups de seus estados. Isso é
/// importante, pois assim é possível passar atributos privados para a Memento.
struct Originator {
    state: u32,
}

impl Originator {
    pub fn save(&self) -> OriginatorBackup {
        OriginatorBackup {
            state: self.state.to_string(),
        }
    }
}

/// Exemplificação de um cliente.
fn main() {

    // Cria o histórico das modificações.
    let mut history = Vec::<OriginatorBackup>::new();

    // Instancia a originadora.
    let mut originator = Originator { state: 0 };

    // Realiza a alteração do estado e o salvamento no historico.
    originator.state = 1;
    history.push(originator.save());

    originator.state = 2;
    history.push(originator.save());

    // Exibe cada um dos históricos.
    for moment in history.iter() {
        moment.print();
    }

    // Realiza um exemplo de restauração, puxando um item da fila do histórico
    // e acionando o método restore(). Note que esse é um modo simples de 
    // realizar o memento, pois há casos onde o objeto originador é acionado a
    // partir do objeto memento, e para isso é necessário anteriormente ser
    // passado uma instância do objeto do originador, ao instanciar um memento.
    // Em todo caso aqui é exemplificado perfeitamente o padrão memento.
    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {}", originator.state);

    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {}", originator.state);
}


