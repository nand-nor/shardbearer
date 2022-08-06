#[derive(Clone)]
pub enum OrderState {
    BOOTSTRAP,  //nodes are still setting up the system, have no state info yet
    RESET,      //reset, reconfiguration, or recovery in process
    ACTIVE,     //nodes are set up, all info complete (Roles, shard services, etc.), order is not currently performing an operation
    LOCKED(OrderStateOp),  //Order is locked for some operation
    ERROR(OrderStateError), //Order is in error state
}

pub enum OrderStateOp {
    MEMBERSHIP_UPDATE, //adding a new node to the order
    MEMBERSHIP_SET,    //establishing entirely new membership group (during bootstrap or subsequent resets)
    REPLICATING,
    VOTING,
    PROBE,              //order is being probed to ensure all order members up and responsive

}

pub enum OrderStateError{
    RECOVERABLE(OrderError),
    RECOVER_RESET, //recoverable with a reset
    UNRECOVERABLE,
}
/// OrderError Enum
/// Indicates the current error at the order level, used to determine if an order-wide
/// reset is required, or a system wide reset. OrderError also will be matched for
/// recoverability
///
/// # description
///
/// @`STATE`: order state across all nodes in order is inconsistent
///
/// @`MEMBERSHIP`: membership data across all nodes in order is inconsistent. This can be
///     either a member is accounted for by some but not all nodes, or a member of the
///     order has gone down. Should lead to a reset at the order level and potentially at the
///     system level, depending
///
/// @`SHARD`: shard data across all nodes in order is inconsistent/incorrect
///
/// @`OTHER`: something else, maybe wrap an error type here?
///
/// # recoverability
///
pub enum OrderError {
    STATE,
    MEMBERSHIP,
    SHARD,
    OTHER
}


impl TryFrom<i32> for OrderState {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OrderState::BOOTSTRAP as i32 => Ok(OrderState::BOOTSTRAP),
            x if x == OrderState::LOCKED as i32 => Ok(OrderState::LOCKED),
            x if x == OrderState::ACTIVE as i32 => Ok(OrderState::ACTIVE),
            x if x == OrderState::RESET as i32 => Ok(OrderState::RESET),
            x if x == OrderState::ERROR as i32 => Ok(OrderState::ERROR),

            _ => Err(()),
        }
    }
}

impl Default for OrderState {
    fn default() -> Self {
        OrderState::BOOTSTRAP
    }
}
