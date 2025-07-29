use std::{
    collections::{HashMap, VecDeque},
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use command::{Command, CommandType};
use game::player_state::PlayerState;
use log::{error, info};

mod command;
mod game;

pub struct Server {
    socket: UdpSocket,
    input_commands: VecDeque<InputCommand>,
    player_state: HashMap<SocketAddr, PlayerState>,
    last_tick: Instant,
    tick_rate: Duration,
    ticks_elapsed: u32,
}

struct InputCommand {
    command: Command,
    src_addr: SocketAddr,
}

impl Server {
    pub fn new(server_addr: Ipv4Addr, port: u16, tick_rate_in_millis: u64) -> io::Result<Self> {
        let addr = SocketAddr::new(IpAddr::V4(server_addr), port);
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            input_commands: VecDeque::new(),
            player_state: HashMap::new(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(tick_rate_in_millis),
            ticks_elapsed: 0,
        })
    }

    pub fn run(&mut self) {
        let mut buffer = [0; 1024];
        loop {
            self.poll_connections(&mut buffer);

            let now = Instant::now();
            if now.duration_since(self.last_tick) >= self.tick_rate {
                self.process_game_tick();
                self.last_tick = now;
                self.ticks_elapsed += 1;
                self.input_commands.clear();
                self.emit_game_state();
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn poll_connections(&mut self, buffer: &mut [u8]) {
        match self.socket.recv_from(buffer) {
            Ok((number_of_bytes, src_addr)) => {
                if let Ok(command) = Command::deserialize(&buffer[..number_of_bytes]) {
                    info!("{} sent {:?}", src_addr, command.command_type);
                    self.input_commands
                        .push_back(InputCommand { command, src_addr });
                }
            }
            Err(e) if e.kind() != io::ErrorKind::WouldBlock => {
                error!("{e}");
            }
            _ => {}
        }
    }

    fn process_game_tick(&mut self) {
        loop {
            let Some(input_command) = self.input_commands.pop_front() else {
                break;
            };

            let command = input_command.command;
            let src_addr = input_command.src_addr;

            match command.command_type {
                CommandType::PlayerJoin => {
                    self.player_state.entry(src_addr).or_default();
                }
                CommandType::PlayerLeave => {
                    self.player_state.remove(&src_addr);
                }
                CommandType::PlayerMove { position, velocity } => {
                    if let Some(player) = self.player_state.get_mut(&src_addr) {
                        player.update(position, velocity);
                    }
                }
                _ => {}
            }
        }
    }

    fn emit_game_state(&self) {
        let collected_states: Vec<PlayerState> = self.player_state.values().cloned().collect();
        self.player_state.iter().for_each(|(src_addr, state)| {
            let game_state = Command {
                command_type: CommandType::Data((state.player_id, collected_states.clone())),
                time: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
            }
            .serialize();
            if let Ok(serialized_state) = game_state {
                if let Ok(num_bytes) = self.socket.send_to(&serialized_state, src_addr) {
                    info!("sent {num_bytes} bytes to {src_addr}");
                } else {
                    error!("failed to send data to {src_addr}");
                }
            } else {
                error!("failed to serialize data to {src_addr}");
            }
        });
    }
}
