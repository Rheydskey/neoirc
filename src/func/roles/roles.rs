pub struct Roles {
    id: i32,
    name: String,
    color: String,
}

impl Roles {
    pub fn new() -> Self {
        Roles {
            id: 0,
            name: String::new(),
            color: String::new(),
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_nick(&self) -> String {
        self.color.clone()
    }
    pub fn set_id(&mut self, id: i32) -> &mut Roles {
        self.id = id;
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut Roles {
        self.name = name;
        self
    }
    pub fn set_nick(&mut self, color: String) -> &mut Roles {
        self.color = color;
        self
    }
}
