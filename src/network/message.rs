use crate::structs::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message{
    Invalid,
    Initialize{nick: String},
    MapUpdate{map: Map},
    Shot{cannon_angle: f32},
    Ping{num: i32},
}