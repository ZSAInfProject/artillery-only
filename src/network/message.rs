use crate::structs::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message{
    Invalid,
    Initialize{nick: String},
    MapUpdate{map: Map},
    Ping{num: i32},
}