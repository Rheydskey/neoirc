use serde::Deserialize;

#[derive(Clone,Deserialize, Debug)]
pub struct UserPassword {

    name: String,
    password: String
}

impl UserPassword {
    pub fn new() -> Self {
        UserPassword {
            name: String::new(),
            password: String::new(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
    pub fn set_name(&mut self, name: String) -> &mut UserPassword {
        self.name = name;
        self
    }
    pub fn set_nick(&mut self, password: String) -> &mut UserPassword {
        self.password = password;
        self
    }
}