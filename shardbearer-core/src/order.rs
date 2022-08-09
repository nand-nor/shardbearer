#![allow(non_camel_case_types)] //someday I will forget all the bad things C taught me
use crate::shard::ShardKeyType;

use std::boxed::Box;

use super::GroupID;

#[derive(Clone)]
pub enum OrderShardAction<K: ShardKeyType> {
    ADD(Box<K>),
    MOVE((Box<K>, GroupID)),
    REMOVE(Box<K>),
    CONFIRM(Box<K>),
    COMMIT_P1(Box<K>),
    COMMIT_P2(Box<K>), //TIMESTAMP((dyn ShardKey, Timestamp))
}

impl<K: ShardKeyType> Default for OrderShardAction<K> {
    fn default() -> Self {
        OrderShardAction::ADD(Box::new(K::default()))
    }
}

#[derive(Clone)]
pub enum OrderState {
    BOOTSTRAP,              //nodes are still setting up the system, have no state info yet
    RESET,                  //reset, reconfiguration, or recovery in process
    ACTIVE, //nodes are set up, all info complete (Roles, shard services, etc.), order is not currently performing an operation
    LOCKED(OrderStateOp), //Order is locked for some operation
    ERROR(OrderStateError), //Order is in error state
}
/// Used to return information if a requesting node in the system gets a message back
/// stating the receiver is locked
#[derive(Clone)]
pub enum OrderStateOp {
    MEMBERSHIP_UPDATE, //adding a new node to the order
    MEMBERSHIP_SET, //establishing entirely new membership group (during bootstrap or subsequent resets)
    REPLICATING,
    VOTING,
    PROBE, //order is being probed to ensure all order members up and responsive
}
/// OrderError Enum
/// Indicates the current error at the order level, used to determine if an order-wide
/// reset is required, or a system wide reset. OrderError also will be matched for
/// recoverability
///
/// # description
///
/// @`RECOVERABLE_STATE`: order state across all nodes in order is inconsistent
///
/// @`RECOVERABLE_MEMBERSHIP`: membership data across all nodes in order is inconsistent. This can be
///     either a member is accounted for by some but not all nodes, or a member of the
///     order has gone down. Should lead to a reset at the order level and potentially at the
///     system level, depending
///
/// @`RECOVERABLE_SHARD`: shard data across all nodes in order is inconsistent/incorrect
///
/// @`RECOVERABLE_OTHER`: something else, maybe wrap an error type here?
///
/// @`RECOVER_RESET`: recoverable with a reset, all above do not require hard reset
///
/// # recoverability
///
#[derive(Clone)]
pub enum OrderStateError {
    RECOVERABLE_STATE,
    RECOVERABLE_MEMBERSHIP,
    RECOVERABLE_SHARD,
    RECOVERABLE_OTHER,
    RECOVER_RESET, //recoverable with a reset
    UNRECOVERABLE,
}

/*
impl TryFrom<i32> for OrderState {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OrderState::BOOTSTRAP as i32 => Ok(OrderState::BOOTSTRAP),
            x if x == OrderState::ACTIVE as i32 => Ok(OrderState::ACTIVE),
            x if x == OrderState::RESET as i32 => Ok(OrderState::RESET),
            x if x == OrderState::ERROR(val) as i32 => return match val.as_ref() {
                OrderStateError::RECOVER_RESET => Ok(OrderState::ERROR(OrderStateError::RECOVER_RESET)),
                OrderStateError::UNRECOVERABLE => Ok(OrderState::ERROR(OrderStateError::UNRECOVERABLE)),
                OrderStateError::RECOVERABLE_STATE => Ok(OrderState::ERROR(OrderStateError::RECOVERABLE_STATE)),
                OrderStateError::RECOVERABLE_MEMBERSHIP => Ok(OrderState::ERROR(OrderStateError::RECOVERABLE_MEMBERSHIP)),
                OrderStateError::RECOVERABLE_SHARD => Ok(OrderState::ERROR(OrderStateError::RECOVERABLE_SHARD)),
                OrderStateError::RECOVERABLE_OTHER => Ok(OrderState::ERROR(OrderStateError::RECOVERABLE_OTHER)),
                _ => Err(()),
            },
            x if x == OrderState::LOCKED(val) as i32 => return match val.as_ref() {
                OrderStateOp::MEMBERSHIP_UPDATE => Ok(OrderState::LOCKED(OrderStateOp::MEMBERSHIP_UPDATE )),
                OrderStateOp::MEMBERSHIP_SET => Ok(OrderState::LOCKED(OrderStateOp::MEMBERSHIP_SET)),
                OrderStateOp::REPLICATING => Ok(OrderState::LOCKED(OrderStateOp::REPLICATING)),
                OrderStateOp::VOTING => Ok(OrderState::LOCKED(OrderStateOp::VOTING)),
                OrderStateOp::PROBE => Ok(OrderState::LOCKED(OrderStateOp::PROBE)),
                _ => Err(()),
            },
            _ => Err(()),

        }
    }
}*/

impl Default for OrderState {
    fn default() -> Self {
        OrderState::BOOTSTRAP
    }
}
