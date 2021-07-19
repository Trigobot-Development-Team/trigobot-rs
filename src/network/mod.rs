mod grpc;

use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::env::var as ENV;
use std::fs;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Mutex, RwLock};

use grpc::broadcast_server::Broadcast;
use grpc::{BroadcastRequest, BroadcastResponse};

use tonic::{Code, Request, Response, Status};

use tokio::task;

const VAR_DOMAINS_FILE: &str = "PEERS_FILE";

/// Uniform Reliable Broadcast implementation

enum ErrorMessage {
    Unauthenticated,
}

impl ErrorMessage {
    pub fn get_message(self) -> String {
        match self {
            Self::Unauthenticated => "Sender is not authenticated",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct BroadcastMessage {
    message: Protocol,
    peer: String,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) enum Protocol {
    Test, // FIXME
}

struct Peer {
    name: String,
    sockets: Vec<SocketAddr>,
}

pub struct Network {
    peers: RwLock<HashMap<String, Peer>>,
    pending: Mutex<HashSet<BroadcastMessage>>,
    delivered: Mutex<HashMap<Protocol, HashSet<String>>>,
    acks: Mutex<HashMap<Protocol, HashSet<String>>>,
}

impl Network {
    pub fn new() -> Self {
        let mut peers = HashMap::new();

        let filename = ENV(VAR_DOMAINS_FILE).expect(&format!(
            "Unknown domains file!\nSet env var {} with the path to the appropriate file",
            VAR_DOMAINS_FILE
        ));

        // Create sockets from domains
        fs::read_to_string(&filename)
            .expect(&format!("Cannot read file {}!", &filename))
            .split("\n")
            .filter_map(|d| match d.to_socket_addrs() {
                Ok(sock) => Some((d, sock.collect::<Vec<SocketAddr>>())),
                Err(e) => {
                    eprintln!("Couldn't resolve domain {}\n{}", d, e);
                    None
                }
            })
            .for_each(|(d, k)| {
                peers.insert(
                    d.to_owned(),
                    Peer {
                        name: d.to_owned(),
                        sockets: k,
                    },
                );
            });

        Network {
            acks: Mutex::new(HashMap::new()),
            delivered: Mutex::new(HashMap::new()),
            peers: RwLock::new(peers),
            pending: Mutex::new(HashSet::new()),
        }
    }

    fn can_deliver(&self, m: Protocol) -> bool {
        (match self.acks.lock() {
            Ok(acks) => match acks.get(&m) {
                Some(acks) => acks.len(),
                None => 0,
            },
            Err(e) => {
                eprintln!("Couldn't get lock on ACKS: {:?}", e);
                0
            }
        }) > (match self.peers.read() {
            Ok(peers) => peers.len(),
            Err(e) => {
                eprintln!("Couldn't get read lock on PEERS: {:?}", e);
                usize::MAX
            }
        } / 2)
    }

    fn broadcast(&self, m: Protocol) {
        todo!()
    }
}

#[tonic::async_trait]
impl Broadcast for Network {
    async fn broadcast(
        &self,
        request: Request<BroadcastRequest>,
    ) -> Result<Response<BroadcastResponse>, Status> {
        let sender_certificates = match request.peer_certs() {
            None => {
                println!(
                    "Unauthenticated broadcast request received from {}",
                    match request.remote_addr() {
                        Some(val) => format!("{}", val),
                        None => "unknown sender".to_string(),
                    }
                );

                return Err(Status::new(
                    Code::Unauthenticated,
                    ErrorMessage::Unauthenticated.get_message(),
                ));
            }
            Some(certs) => certs,
        };

        // FIXME: parse certificates to get peer
        let peer = Peer {
            name: "dummy".to_string(),
            sockets: Vec::new(),
        };

        // FIXME: Deserialize message
        let message = Protocol::Test;

        // Add ack from process
        let acked = match self.acks.lock() {
            Ok(mut acks) => match acks.get_mut(&message) {
                None => {
                    let mut set = HashSet::new();
                    set.insert(peer.name.clone());

                    acks.insert(message.clone(), set);

                    false
                }
                Some(acks) => acks.insert(peer.name.clone()),
            },
            Err(e) => return Err(Status::new(Code::Unknown, format!("{:#?}", e))),
        };

        if !acked {
            match self.pending.lock() {
                Ok(mut pending) => {
                    pending.insert(BroadcastMessage {
                        peer: peer.name,
                        message,
                    });
                }
                Err(e) => return Err(Status::new(Code::Unknown, format!("{:#?}", e))),
            };

            // TODO: Broadcast message
        }

        // TODO: Check if message can be delivered

        // TODO: Deliver message
        task::spawn(async {
            println!("ALL OK!");
        });

        Ok(Response::new(BroadcastResponse {}))
    }
}
