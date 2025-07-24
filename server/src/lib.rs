use std::{
    collections::{HashMap, VecDeque},
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::{Duration, Instant},
};

mod packet;

pub struct Server {
    socket: UdpSocket,
    input_commands: HashMap<SocketAddr, VecDeque<()>>, // TODO
    player_state: HashMap<SocketAddr, ()>,             // TODO
    last_tick: Instant,
    tick_rate: Duration,
    ticks_elapsed: u32,
}

struct InputCommand {}

impl Server {
    pub fn new(server_addr: Ipv4Addr, port: u16, tick_rate_in_millis: u64) -> io::Result<Self> {
        let addr = SocketAddr::new(IpAddr::V4(server_addr), port);
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            input_commands: HashMap::new(),
            player_state: HashMap::new(),
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

    fn poll_connections(&self, buffer: &mut [u8]) -> io::Result<()> {
        loop {
            match self.socket.recv_from(buffer) {
                Ok((number_of_bytes, src_addr)) => {
                    println!(
                        "recieved {:?} from {:?}",
                        &buffer[..number_of_bytes],
                        src_addr
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

    fn process_game_tick(&self) {}
}
