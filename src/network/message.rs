
#[derive(Serialize, Deserialize, Debug)]
pub enum Message{
    Invalid,
    Ping{num: i32},
}