use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use directories::ProjectDirs;

use crate::network::packets::{IdentifyPacket, Packet, RawPacket};

pub mod packets;

pub struct DaemonSockClient {
    stream: UnixStream,
}

impl DaemonSockClient {
    pub fn new() -> Self {
        let sock_path = DaemonSockClient::get_path().expect("Failed to get daemon sock path");

        let stream = UnixStream::connect(sock_path).expect("Failed to connect to daemon socket");

        let mut dsc = DaemonSockClient { stream };

        dsc.write_packet(Box::new(IdentifyPacket {
            client_type: "daemon".to_string(),
        }));

        dsc
    }

    pub fn write_packet(&mut self, packet: Box<dyn Packet>) {
        let serialized = packet.serialize();
        let packet = RawPacket {
            id: packet.get_id() as u8,
            data: serialized,
        };
        self.stream
            .write_all(&packet.serialize())
            .expect("Failed to write to daemon socket");
    }

    pub fn get_path() -> Result<PathBuf, &'static str> {
        if let Some(proj_dirs) = ProjectDirs::from("dev", "floffah", "mcproc") {
            let sock_dir = proj_dirs.runtime_dir();
            let sock_dir = match sock_dir {
                Some(dir) => {
                    if !dir.exists() {
                        std::fs::create_dir_all(dir).expect("Failed to create runtime directory");
                    }

                    dir
                }
                None => {
                    let data_dir = proj_dirs.data_dir();

                    if !data_dir.exists() {
                        std::fs::create_dir_all(data_dir).expect("Failed to create data directory");
                    }

                    data_dir
                }
            };

            let daemon_sock_path = sock_dir.join("daemon.sock");

            return Ok(daemon_sock_path);
        }

        return Err("Failed to get daemon sock path");
    }
}
