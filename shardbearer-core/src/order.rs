use crate::shard::{Shard, ShardKey};
use std::convert::TryFrom;

#[derive(Clone)]
pub enum RadiantOrderState {
    INACTIVE, //nodes are still setting up the system, have no state info yet
    VOTING,   //After the inactive state we must vote to fully be setup and active. There are
    //multiple rounds of voting so may need to split this into a state for each voting round?
    ACTIVE,    //nodes are set up, all info complete (Roles, shard services, etc.)
    RESETLOCK, //reconfiguration in process
    ERROR,
}

impl TryFrom<i32> for RadiantOrderState {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == RadiantOrderState::INACTIVE as i32 => Ok(RadiantOrderState::INACTIVE),
            x if x == RadiantOrderState::VOTING as i32 => Ok(RadiantOrderState::VOTING),
            x if x == RadiantOrderState::ACTIVE as i32 => Ok(RadiantOrderState::ACTIVE),
            x if x == RadiantOrderState::RESETLOCK as i32 => Ok(RadiantOrderState::RESETLOCK),
            x if x == RadiantOrderState::ERROR as i32 => Ok(RadiantOrderState::ERROR),

            _ => Err(()),
        }
    }
}

impl Default for RadiantOrderState {
    fn default() -> Self {
        RadiantOrderState::INACTIVE
    }
}

#[derive(Clone)]
pub enum OrderShardAction {
    ADD(ShardKey),
    REMOVE(ShardKey),
}

impl Default for OrderShardAction {
    fn default() -> Self {
        OrderShardAction::ADD(0)
    }
}
