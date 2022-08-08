#[derive(Clone, Debug)]
pub enum RadiantState {
    RESET,
    UNASSOCIATED,
    ASSOCIATED,
    LOCKED,     //replicating, voting, or performing some op that requires atomicity
    ERROR(RadiantStateError),
}

//TODO add more of these later
#[derive(Clone, Debug, Error)]
pub enum RadiantStateError {
    #[error("radiant state error")]
    ERROR,
}

impl Default for RadiantState {
    fn default() -> Self {
        RadiantState::UNASSOCIATED
    }
}


pub trait RadiantStateMachine {
    type RadiantState;
    type SystemState;
    type ClusterState;
    type OrderState;

    fn order_state(&self) -> Self::OrderState;
    fn update_order_state(&mut self, state: Self::OrderState);
    fn update_cluster_state(&mut self, state: Self::ClusterState);
    fn cluster_state(&self) -> Self::ClusterState;

    fn update_radiant_state(&mut self, state: Self::RadiantState);
    fn radiant_state(&self) -> Self::RadiantState;

    fn update_system_state(&mut self, state: Self::SystemState);
    fn system_state(&self) -> Self::SystemState;
}