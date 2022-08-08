#[derive(Clone)]
pub enum SysState {
    READY,
    LOCKED,
    ERROR,
    INIT,
}

impl Default for SysState {
    fn default() -> Self {
        SysState::INIT
    }
}

pub trait RadiantSystem{

}

pub trait HeraldSystem{

}

pub trait ShardbearerSystem{

}