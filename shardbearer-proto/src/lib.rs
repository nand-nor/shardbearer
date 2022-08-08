extern crate futures;
extern crate grpcio;
extern crate protobuf;

pub mod herald {
    include!(concat!("../proto/herald/mod.rs"));
}

pub mod radiant {
    include!(concat!("../proto/radiant/mod.rs"));
}
/*
pub mod shard {
    include!(concat!("../proto/shard/mod.rs"));
}
*/

pub mod bondsmith {
    include!(concat!("../proto/bondsmith/mod.rs"));
}

pub mod common {
    include!(concat!("../proto/mod.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
