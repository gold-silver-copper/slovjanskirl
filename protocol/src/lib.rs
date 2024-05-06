use bevy_rtc::protocol::Protocol;
use serde::{Deserialize, Serialize};
use slov_common::{MyPoint, EntityID};

// Used by painting demo

#[derive(Protocol, Serialize, Deserialize, Debug, Clone)]
pub struct DrawLinePayload {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Protocol, Serialize, Deserialize, Debug, Clone)]
pub struct LoginLoad {
    pub username: String,
    pub password: String,
}
#[derive(Protocol, Serialize, Deserialize, Debug, Clone)]
pub struct LoginAnswer {
    pub player_id: EntityID,
    pub player_location: MyPoint,
}

// Used by ping demo

#[derive(Protocol, Serialize, Deserialize, Debug, Clone)]
pub struct PingPayload;

#[derive(Protocol, Serialize, Deserialize, Debug, Clone)]
pub struct PongPayload;