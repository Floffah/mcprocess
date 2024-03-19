use log::error;
use tokio::net::UnixListener;
use tokio::task::JoinHandle;

pub(crate) fn start_sock() -> JoinHandle<()> {
    tokio::spawn(async {
        let sock_path = core::network::DaemonSockClient::get_path().unwrap();

        if sock_path.exists() {
            std::fs::remove_file(&sock_path).unwrap();
        }

        let listener = UnixListener::bind(sock_path).unwrap();

        println!("Socket started - accepting clients");

        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {

                }
                Err(e) => {
                    error!("Failed to accept client: {:?}", e);
                }
            }
        }
    })
}