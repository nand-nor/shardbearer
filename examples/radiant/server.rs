use shardbearer;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    shardbearer::server::server_main::<u64,u64>()?;
    Ok(())
}
