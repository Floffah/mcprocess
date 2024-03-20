use std::time::SystemTime;

use tokio::sync::mpsc;

use crate::concurrency::DaemonEvent;
use crate::servers::start_servers;
use crate::sock::start_sock;

mod concurrency;
mod servers;
mod sock;

#[tokio::main]
async fn main() -> Result<(), String> {
    let global_data = core::data::global::read_global_data().unwrap();

    setup_logger().unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel::<DaemonEvent>();

    let mut servers = start_servers(&global_data).unwrap();

    let ctrlc_tx = tx.clone();
    ctrlc::set_handler(move || ctrlc_tx.send(DaemonEvent::StopDaemon).unwrap()).unwrap();

    let sock_tx = tx.clone();
    let sock_future = start_sock(sock_tx);

    loop {
        match rx.recv().await {
            Some(event) => {
                match event {
                    DaemonEvent::StopDaemon => {
                        sock_future.abort();
                        for server in servers.iter_mut() {
                            server.process.kill().unwrap();
                        }
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    return Ok(());
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(
            core::data::logs::get_daemon_log_path().unwrap(),
        )?)
        .apply()?;
    Ok(())
}
