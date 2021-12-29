use shardbearer;


//this illustrates a very simple custom entrypoint
//that builds a tokio runtime with the default config
//instead of using the optional tokio config vals
//provided in the config toml
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    if let Some(toml) = std::env::args().nth(1) {
        let cfg = match shardbearer::config::parse_cfg(&toml) {
            Ok(cfg) => cfg,
            Err(e) => {
                tracing::error!("RadiantServer: Invalid config file provided: {:?}", e.to_string());
                std::process::exit(1);
            }
        };

        let tokio_cfg = cfg.runtime_cfg();

        let rt = match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(tokio_cfg.num_threads())
            .thread_stack_size(tokio_cfg.thread_stack_size())
            .thread_name(tokio_cfg.runtime_name())
            .enable_all()
            .build(){
            Ok(t)=>t,
            Err(e)=>{
                tracing::error!("RadiantServer: Unable to build requested tokio runtime: {:?}", e.to_string());
                std::process::exit(1);
            }
        };
        rt.block_on(async move {
            if let Err(e) = shardbearer::server::radiant_server::<u64,u64>(cfg).await {
                tracing::error!("RadiantServer: error running server {:?}", e.to_string());

            }

        });

    } else {
        tracing::error!("No config file provided");
        std::process::exit(1);
    }

    Ok(())
}

