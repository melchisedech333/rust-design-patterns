
/// Componente base que será utilizado tanto pelos traits File quanto Folder.

mod file;
mod folder;

pub use file::File;
pub use folder::Folder;

pub trait Component {
    fn search(&self, keyword: &str);
}


