use crate::structs::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Invalid,
    Disconnect,
    Initialize { nick: String },
    MapUpdate { map: Map },
    Shot { cannon_angle: f32 },
    Ping { num: i32 },
}
