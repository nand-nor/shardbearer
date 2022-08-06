#[derive(Clone, Debug)]
pub enum RadiantState {
    RESET,
    UNASSOCIATED,
    ASSOCIATED,
    LOCKED,     //replicating, voting, or performing some op that requires atomicity
    ERROR(RadiantStateError),
}

