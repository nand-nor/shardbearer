extern crate futures;
extern crate grpcio;
extern crate protobuf;

pub mod herald {
    include!(concat!("../proto/herald/bondsmith"));
}

pub mod radiant {
    include!(concat!("../proto/radiant/bondsmith"));
}

pub mod shard {
    include!(concat!("../proto/shard/bondsmith"));
}

pub mod controller {
    include!(concat!("../proto/bondsmith/bondsmith"));
}

pub mod common {
    include!(concat!("../proto/bondsmith"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
