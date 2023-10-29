
/// Exemplo de como realizar a implementação de um iterador próprio/customizado.

/// Define estrutura de dados que será utilizada.
pub struct UserCollection {
    users: [&'static str; 3],
}

/// A custom collection contains an arbitrary user array under the hood.
/// 
/// Implementação concreta da collection.
impl UserCollection {
    /// Returns a custom user collection.
    pub fn new() -> Self {
        Self {
            users: ["Alice", "Bob", "Carl"],
        }
    }

    /// Returns an iterator over a user collection.
    ///
    /// The method name may be different, however, `iter` is used as a de facto
    /// standard in a Rust naming convention.
    /// 
    /// Note que o iter() retorna uma instância do UserIterator, ele será utilizado
    /// no lado do cliente.
    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index: 0,
            user_collection: self,
        }
    }
}

/// UserIterator allows sequential traversal through a complex user collection
/// without exposing its internal details.
/// 
/// Implementação concreta do iterador.
pub struct UserIterator<'a> {
    index: usize, // Representa o index atual.
    user_collection: &'a UserCollection, // Aponta para a coleção que está sendo iterada.
}

/// `Iterator` is a standard interface for dealing with iterators
/// from the Rust standard library.
impl Iterator for UserIterator<'_> {
    type Item = &'static str;

    /// A `next` method is the only `Iterator` trait method which is mandatory to be
    /// implemented. It makes accessible a huge range of standard methods,
    /// e.g. `fold`, `map`, `for_each`.
    /// 
    /// Percorre a estrutura de dados retornando cada um de seus elementos.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index]);
            self.index += 1;
            return user;
        }

        None
    }
}


