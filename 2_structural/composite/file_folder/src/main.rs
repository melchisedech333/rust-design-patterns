
mod file_system;

use file_system::{Component, File, Folder};

fn main() {
    
    let mut folder1 = Folder::new("Folder 1");
    folder1.add(File::new("File 1"));

    let mut folder2 = Folder::new("Folder 2");
    folder2.add(File::new("File 2"));
    folder2.add(File::new("File 3"));
    folder2.add(folder1); // Cria um padrão de estrutura em árvore.

    // Realiza a busca de modo recursivo nos elementos da árvore.
    folder2.search("Iesus Hominum Salvator");
}


