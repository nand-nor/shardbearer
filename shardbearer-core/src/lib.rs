//pub mod controller;
pub mod herald;
//pub mod membership;
pub mod cluster;
pub mod order;
pub mod radiant;
pub mod raft;
pub mod shard;
pub mod system;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
