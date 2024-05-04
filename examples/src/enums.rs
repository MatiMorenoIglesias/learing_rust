pub enum UserType {
    Admin (User),
    Normal (User),
}

struct User {
    id: i128,
    email: String,
    username: String,
    password: String,
    active: bool
}