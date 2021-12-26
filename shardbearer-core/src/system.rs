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

pub trait System: Default {
    type DiskSpace;
    type SystemState;
    fn update_disk_space(&mut self, change: Self::DiskSpace);
    fn check_disk_space(&self) -> Self::DiskSpace;
    fn report_state(&self) -> Self::SystemState;
    fn update_state(&mut self, state: Self::SystemState);
}

#[derive(Default)]
pub struct RadiantSystem {
    space: u64,
    state: SysState,
}

impl System for RadiantSystem {
    type DiskSpace = u64;
    type SystemState = SysState;

    fn update_disk_space(&mut self, change: Self::DiskSpace) {
        self.space = change;
    }

    fn check_disk_space(&self) -> Self::DiskSpace {
        self.space
    }

    fn report_state(&self) -> Self::SystemState {
        self.state.clone()
    }
    fn update_state(&mut self, state: Self::SystemState) {
        self.state = state
    }
}
