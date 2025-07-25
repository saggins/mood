use std::{
    collections::{HashMap, VecDeque},
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::{Duration, Instant},
};

use command::{Command, CommandType};

mod command;
mod game;

pub struct Server {
    socket: UdpSocket,
    input_commands: HashMap<SocketAddr, VecDeque<Command>>,
    player_states: HashMap<SocketAddr, ()>, // TODO
    last_tick: Instant,
    tick_rate: Duration,
    ticks_elapsed: u32,
}

impl Server {
    pub fn new(server_addr: Ipv4Addr, port: u16, tick_rate_in_millis: u64) -> io::Result<Self> {
        let addr = SocketAddr::new(IpAddr::V4(server_addr), port);
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            input_commands: HashMap::new(),
            player_states: HashMap::new(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(tick_rate_in_millis),
            ticks_elapsed: 0,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1024];
        loop {
            self.poll_connections(&mut buffer)?;

            let now = Instant::now();
            if now.duration_since(self.last_tick) >= self.tick_rate {
                self.process_game_tick();
                self.last_tick = now;
                self.ticks_elapsed += 1;
            }
            std::thread::sleep(Duration::from_micros(100));
        }
    }

    fn poll_connections(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        loop {
            match self.socket.recv_from(buffer) {
                Ok((number_of_bytes, src_addr)) => {
                    let Ok(command) = Command::deserialize(&buffer[..number_of_bytes]) else {
                        println!("Invalid data recieved from: {src_addr}");
                        break;
                    };

                    match command.command_type {
                        CommandType::PlayerJoin => {
                            println!("{src_addr} joined the lobby");
                            self.input_commands.insert(src_addr, VecDeque::new());
                            self.player_states.insert(src_addr, ());
                        }
                        CommandType::PlayerLeave => {
                            println!("{src_addr} left the lobby");
                            self.input_commands.remove(&src_addr);
                            self.player_states.remove(&src_addr);
                        }
                        _ => {}
                    }

                    println!(
                        "recieved command: {:?} from player: {}",
                        command.command_type, command.player_id
                    );
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => {
                    println!("{e}");
                }
            }
        }
        Ok(())
    }

    fn process_game_tick(&mut self) {}
}
