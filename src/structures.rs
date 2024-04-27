pub struct User {
    pub actif: bool,
    pub pseudo: String,
    pub email: String,
    pub nombre_de_connexions: u64,
}

pub fn create_user(pseudo: String, email: String) -> User {
    User {
        actif: true,
        pseudo,
        email,
        nombre_de_connexions: 1,
    }
}
