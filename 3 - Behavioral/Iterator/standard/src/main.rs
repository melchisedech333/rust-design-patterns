
/// Exemplo de uso com os próprios recursos da linguagem.

fn main() {
    let array = &[1, 2, 3];
    let iterator = array.iter();
    
    // Traversal over each element of the vector.
    iterator.for_each(|e| print!("{}, ", e));    
}


