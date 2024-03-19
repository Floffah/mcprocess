use clap::Parser;

mod args;
mod cmds;

fn main() {
    let cli = args::Cli::parse();

    let data = core::data::global::read_global_data();

    if let Err(error) = data {
        panic!("Failed to read global data: {:?}", error);
    }

    let data = data.unwrap();

    match &cli.command {
        args::Commands::Import { path } => {
            crate::cmds::import::run(&data, path);
        }
        args::Commands::Start { name } => {
            core::network::DaemonSockClient::new();
        }
    }
}
