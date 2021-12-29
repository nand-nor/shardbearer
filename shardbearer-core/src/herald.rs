/*#[derive(Clone)]
pub enum HeraldRole {
    VOTER,
    CONTROLLER,
}

impl Default for HeraldRole {
    fn default() -> Self {
        HeraldRole::VOTER
    }
}
*/

pub trait Herald {
    type ControllerId;

    fn elect_controller(&mut self) -> Self::ControllerId;
    fn controller(&mut self) -> Self::ControllerId;
}
