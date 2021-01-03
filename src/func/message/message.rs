pub struct Message {
    id: i32,
    body: String,
}

impl Message {
    pub fn new() -> Self {
        Message {
            id: 0,
            body: String::new(),
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn set_id(&mut self, id: i32) -> &mut Message {
        self.id = id;
        self
    }
    pub fn set_body(&mut self, body: String) -> &mut Message {
        self.body = body;
        self
    }
}
