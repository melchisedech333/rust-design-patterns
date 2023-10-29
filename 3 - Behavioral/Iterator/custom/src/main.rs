
/// Exemplo de aplicação cliente fazendo uso do iterador customizado.

mod users;
use crate::users::UserCollection;

fn main() {
    print!("Iterators are widely used in the standard library: ");

    let array = &[1, 2, 3];

    // Traversal over each element of the array.
    array.iter().for_each(|e| print!("{} ", e));

    println!("\n\nLet's test our own iterator.\n");

    let users = UserCollection::new();
    let mut iterator = users.iter();

    println!("1nd element: {:?}", iterator.next());
    println!("2nd element: {:?}", iterator.next());
    println!("3rd element: {:?}", iterator.next());
    println!("4th element: {:?}", iterator.next());

    print!("\nAll elements in user collection: ");

    // Note que o uso se dá literalmente como é realizado com o iterador 
    // padrão da linguagem.
    users.iter().for_each(|e| print!("{} ", e));

    println!();
}


