pub struct Channel {
    id: i32,
    name: String,
    channel_type: ChannelType,
}
#[derive(Clone)]
pub enum ChannelType {
    Voice,
    Text,
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            id: 0,
            name: String::new(),
            channel_type: ChannelType::Text,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_type(&self) -> ChannelType {
        self.channel_type.clone()
    }
    pub fn set_id(&mut self, id: i32) -> &mut Channel {
        self.id = id;
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut Channel {
        self.name = name;
        self
    }
    pub fn set_type(&mut self, channel_type: ChannelType) -> &mut Channel {
        self.channel_type = channel_type;
        self
    }
}
