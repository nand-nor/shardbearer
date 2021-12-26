pub type MemberID = u64;
pub type GroupID = u64;

//#[derive(Default)]
pub enum MembershipLevel {
    RADIANT(MemberID), //query the state of the radiant with the specified MemberID
    ORDER(GroupID),    //query all radiants in the order specified by the GID
    HERALD(GroupID),   //query the herald of the order specified by the GID
    SHARDHERALD,       //query who the current shard controlling herald is
    ALL,               //query all existing radiants known to the system
}

impl Default for MembershipLevel {
    fn default() -> Self {
        MembershipLevel::RADIANT(0)
    }
}
#[derive(Clone)]
pub enum Membership {
    RADIANT,
    HERALD,
    SHARDHERALD,
    UNASSOCIATED,
}

impl Default for Membership {
    fn default() -> Self {
        Membership::RADIANT
    }
}

pub trait Role: Default {
    type RoleState;
    type RoleId;
    fn get_state(&self) -> Self::RoleState;
    fn set_state(&mut self, state: Self::RoleState);
    fn set_id(&mut self, id: Self::RoleId);
    fn get_id(&self) -> Self::RoleId;
}

#[derive(Default)]
pub struct RadiantMembership {
    state: Membership,
    id: MemberID,
}

impl RadiantMembership {
    pub fn default() -> Self {
        Self {
            state: Membership::UNASSOCIATED,
            id: 0,
        }
    }
}

impl Role for RadiantMembership {
    type RoleState = Membership;
    type RoleId = MemberID;

    fn get_state(&self) -> Self::RoleState {
        self.state.clone()
    }
    fn set_state(&mut self, state: Self::RoleState) {
        self.state = state
    }
    fn set_id(&mut self, id: Self::RoleId) {
        self.id = id
    }
    fn get_id(&self) -> Self::RoleId {
        self.id
    }
}
