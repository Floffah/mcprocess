use log::error;
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use crate::concurrency::DaemonEvent;
use core::network::packets as packets;

struct Connection {
    identity: String
}

pub(crate) fn start_sock(tx: UnboundedSender<DaemonEvent>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let sock_path = core::network::DaemonSockClient::get_path().unwrap();

        if sock_path.exists() {
            std::fs::remove_file(&sock_path).unwrap();
        }

        let listener = UnixListener::bind(sock_path).unwrap();

        println!("Socket started - accepting clients");

        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => {
                    let tx = tx.clone();
                    tokio::spawn(async move {
                        let mut conn = Connection {
                            identity: "unknown".to_string()
                        };
                        
                        loop {
                            let len = stream.read_u32().await.unwrap();
                            let mut buf = vec![0; len as usize];
                            stream.read_exact(&mut buf).await.unwrap();

                            let raw_packet = packets::RawPacket::deserialize(len, &buf);
                            let packet = raw_packet.get_packet();

                            handle_packet(packet, &raw_packet, &mut conn, &tx);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept client: {:?}", e);
                }
            }
        }
    })
}

fn handle_packet(packet: Box<dyn packets::Packet>, raw_packet: &packets::RawPacket, conn: &mut Connection, tx: &UnboundedSender<DaemonEvent>) {
    match packet.get_id() {
        packets::PacketId::Identify => {
            let packet = packets::IdentifyPacket::deserialize(raw_packet);
            conn.identity = packet.client_type;
        }
    }
}