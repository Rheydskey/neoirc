use async_std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone, Debug)]
pub struct IRCServer {
    pub ip: SocketAddr,
    pub user: Vec<User>,
    pub channel: Vec<Channel>,
}
impl IRCServer {
    pub fn new() -> Self {
        IRCServer {
            ip: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555),
            user: Vec::new(),
            channel: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> &mut IRCServer {
        self.user.push(user);
        self
    }
    pub fn add_channel(&mut self, channel: Channel) -> &mut IRCServer {
        self.channel.push(channel);
        self
    }
}
#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub ip: SocketAddr,
    pub name: String,
    pub nick: String,
}
#[derive(Clone, Debug)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub channel_type: ChannelType,
}
#[derive(Clone, Debug)]
pub enum ChannelType {
    Text,
    Voice,
}
