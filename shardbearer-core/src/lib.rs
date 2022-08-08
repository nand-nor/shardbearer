pub mod consensus;
pub mod herald;
pub mod order;
pub mod radiant;
pub mod shard;
pub mod msg;


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
