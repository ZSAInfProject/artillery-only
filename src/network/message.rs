
#[derive(Serialize, Deserialize, Debug)]
pub enum Message{
    Invalid,
    Initialize{nick: String},
    Ping{num: i32},
}