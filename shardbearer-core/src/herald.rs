#[derive(Clone)]
pub enum HeraldState {
    VOTER,
    CONTROLLER,
}

impl Default for HeraldState {
    fn default() -> Self {
        HeraldState::VOTER
    }
}

pub trait Herald {
    type ControllerId;

    fn elect_controller(&mut self) -> Self::ControllerId;
    fn controller(&mut self) -> Self::ControllerId;
}
