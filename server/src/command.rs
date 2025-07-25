use std::{
    cell::RefCell,
    error::Error,
    io::{Cursor, Read},
    rc::Rc,
};

use crate::game::player_state::PlayerState;

pub struct Command {
    pub command_type: CommandType,
    pub time: u128,
}

#[derive(Debug)]
pub enum CommandType {
    PlayerJoin,
    PlayerLeave,
    PlayerMove {
        position: [f32; 3],
        velocity: [f32; 3],
    },
    Data(Vec<PlayerState>),
}

impl Command {
    pub fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    /// id, command type, [optional command data], timestamp
    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn Error>> {
        if data.len() < 10 {
            return Err("Data too short".into());
        }

        let mut cursor = Cursor::new(data);

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

        let time = Self::read_u128_le(&mut cursor)?;

        Ok(Self { command_type, time })
    }

    fn read_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, Box<dyn Error>> {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_f32_le(cursor: &mut Cursor<&[u8]>) -> Result<f32, Box<dyn Error>> {
        todo!()
    }

    fn read_u128_le(cursor: &mut Cursor<&[u8]>) -> Result<u128, Box<dyn Error>> {
        todo!()
    }
}
