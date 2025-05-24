use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum MessageType {
    JoinRequest {
        cert: String,
        ip: String
    },
    JoinResponse {
        accepted: bool,
        reason: Option<String>,
    },
    Disconnect {
        reason: String,
    },
    Chat {
        from: String,
        content: String,
    },
    Ping,
}
