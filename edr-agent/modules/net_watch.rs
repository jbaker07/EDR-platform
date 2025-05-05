use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NetConnection {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub pid: Option<u32>,
}

pub fn collect_net_connections() -> Vec<NetConnection> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let mut connections = Vec::new();

    if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
        for socket in sockets {
            let (proto, local, remote, pid) = match socket.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp) => (
                    "TCP",
                    format!("{}:{}", tcp.local_addr, tcp.local_port),
                    format!("{}:{}", tcp.remote_addr, tcp.remote_port),
                    socket.associated_pids.get(0).cloned(),
                ),
                ProtocolSocketInfo::Udp(udp) => (
                    "UDP",
                    format!("{}:{}", udp.local_addr, udp.local_port),
                    "N/A".to_string(),
                    socket.associated_pids.get(0).cloned(),
                ),
            };

            connections.push(NetConnection {
                protocol: proto.to_string(),
                local_addr: local,
                remote_addr: remote,
                pid,
            });
        }
    }

    connections
}
