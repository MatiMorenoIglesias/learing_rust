pub struct Customer {
    name: String,
    is_active: bool
}

impl Customer {
    pub fn new(name: String) -> Customer{
        Customer{
            name,
            is_active: true
        }
    }
    pub fn is_active(&self) -> String {
        if self.is_active == true {
            return String::from("activo");
        }
        return String::from("inactivo");
    }
    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}