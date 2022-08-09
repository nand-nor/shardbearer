#![allow(dead_code)] //just while features are still being implemented
#![allow(unused_variables)] //^^
#![allow(clippy::result_unit_err)] //need to open git issue to impl custom error types

pub mod bondsmith;
pub mod consensus;
pub mod herald;
pub mod order;
pub mod radiant;
pub mod shard;
pub mod sys;

#[derive(Clone)]
pub enum RadiantRole {
    UNASSOCIATED,
    MEMBER(OrderRole),
}

impl Default for RadiantRole {
    fn default() -> Self {
        RadiantRole::UNASSOCIATED
    }
}

#[derive(Clone)]
pub enum OrderRole {
    VOTER,
    HERALD(HeraldRole),
}

impl Default for OrderRole {
    fn default() -> Self {
        OrderRole::VOTER
    }
}

#[derive(Clone)]
pub enum HeraldRole {
    VOTER,
    BONDSMITH,
}

impl Default for HeraldRole {
    fn default() -> Self {
        HeraldRole::VOTER
    }
}

pub type MemberID = u64;
pub type GroupID = u64;
pub type RadiantID = u64;

#[derive(Default)]
pub struct Timestamp {
    seconds: u64,
    nanos: u32,
}
