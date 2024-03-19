use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use directories::ProjectDirs;

pub enum PacketId {
    Identify = 0x00,
}

pub struct DaemonSockClient {
    stream: UnixStream,
}

impl DaemonSockClient {
    pub fn new() -> Self {
        let sock_path = DaemonSockClient::get_path().expect("Failed to get daemon sock path");

        let stream = UnixStream::connect(sock_path).expect("Failed to connect to daemon socket");

        let mut dsc = DaemonSockClient {
            stream,
        };

        dsc.write_packet(PacketId::Identify, b"CLI".to_vec());

        dsc
    }

    pub fn write_packet(&mut self, id: PacketId, packet: Vec<u8>) {
        let mut buf: Vec<u8> = vec![];
        buf.push((packet.len()) as u8 + 1u8);
        buf.push(id as u8);
        buf.extend(packet);

        self.stream.write_all(&buf).expect("Failed to write packet to daemon");
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
                },
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

