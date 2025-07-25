use std::{
    error::Error,
    io::{Cursor, Read},
};

pub struct Command {
    pub player_id: u8,
    pub command_type: CommandType,
    pub time: u64,
}

#[derive(Debug)]
pub enum CommandType {
    PlayerJoin,
    PlayerLeave,
    PlayerMove {
        position: [f32; 3],
        velocity: [f32; 3],
    },
    PlayerChatMessage(String),
    PlayerFireBullet {
        start: [f32; 3],
        direction: [f32; 3],
    },
}

impl Command {
    pub fn serialize(&self) -> &[u8] {
        todo!()
    }

    /// id, command type, [optional command data], timestamp
    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn Error>> {
        if data.len() < 10 {
            return Err("Data too short".into());
        }

        let mut cursor = Cursor::new(data);

        let player_id = Self::read_u8(&mut cursor)?;
        let command_type = match Self::read_u8(&mut cursor)? {
            0 => CommandType::PlayerJoin,
            1 => CommandType::PlayerLeave,
            2 => {
                let pos_x = Self::read_f32_le(&mut cursor)?;
                let pos_y = Self::read_f32_le(&mut cursor)?;
                let pos_z = Self::read_f32_le(&mut cursor)?;

                let vel_x = Self::read_f32_le(&mut cursor)?;
                let vel_y = Self::read_f32_le(&mut cursor)?;
                let vel_z = Self::read_f32_le(&mut cursor)?;
                CommandType::PlayerMove {
                    position: [pos_x, pos_y, pos_z],
                    velocity: [vel_x, vel_y, vel_z],
                }
            }
            _ => {
                return Err("Invalid command type".into());
            }
        };

        let time = Self::read_u64_le(&mut cursor)?;

        Ok(Self {
            player_id,
            command_type,
            time,
        })
    }

    fn read_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, Box<dyn Error>> {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_f32_le(cursor: &mut Cursor<&[u8]>) -> Result<f32, Box<dyn Error>> {
        todo!()
    }

    fn read_u64_le(cursor: &mut Cursor<&[u8]>) -> Result<u64, Box<dyn Error>> {
        todo!()
    }
}
