use std::{
    collections::HashMap,
    error::Error,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::{SystemTime, UNIX_EPOCH},
};

use command::Command;
use player_state::PlayerState;
use uuid::Uuid;

pub mod command;
pub mod player_state;

pub struct Network {
    socket: UdpSocket,
    player_state: HashMap<Uuid, PlayerState>,
}

impl Network {
    pub fn new(ip_addr: Ipv4Addr, port: u16) -> io::Result<Self> {
        let addr = SocketAddr::new(IpAddr::V4(ip_addr), port);
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            player_state: HashMap::new(),
        })
    }

    pub fn poll(&self) -> Option<()> {
        todo!();
    }

    pub fn connect(&self) -> io::Result<()> {
        let connect_command = Command {
            command_type: command::CommandType::PlayerJoin,
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        };
        self.socket.send(&connect_command.serialize().unwrap())?;
        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        todo!();
    }

    pub fn send(&self) {
        todo!();
    }
}
