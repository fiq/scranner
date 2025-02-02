use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::error::Error;
use chrono::{Utc, DateTime};

pub struct PacketInfo {
    pub date: DateTime<Utc>,
    pub src_mac: String,
    pub dst_mac: String,
    pub src_ip_v4: String,
    pub dst_ip_v4: String,
    pub src_port: u16,
    pub dst_port: u16,
}

pub fn sniff(ifname: String, sample_size: i32) -> Result<Vec<PacketInfo>, Box<dyn Error>> {
    println!("Sniffing interface {}", ifname);
    let interface_name_match = |iface: &NetworkInterface| iface.name == ifname;
    let interfaces = datalink::interfaces();
    let interface = match interfaces.into_iter().find(interface_name_match) {
        Some(iface) => iface,
        None => return Err(format!("Interface {} not found", ifname).into()),
    };

    let mut rx = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => return Err("Unhandled channel type".into()),
        Err(e) => return Err(Box::new(e)),
    };

    let mut packets = Vec::<PacketInfo>::new();
    let mut counter = 0;
    while counter < sample_size {
        let packet = rx.next();
        counter += 1;
        if let Some(eth_packet) = EthernetPacket::new(packet?) {
            println!("Captured Ethernet packet: {:?}", eth_packet);

            if eth_packet.get_ethertype() == EtherTypes::Ipv4 {
                process_ipv4_packet(&mut packets, &eth_packet);
            } else {
                packets.push(PacketInfo {
                    date: Utc::now(),
                    src_mac: eth_packet.get_source().to_string(),
                    dst_mac: eth_packet.get_destination().to_string(),
                    src_ip_v4: "N/A".to_string(),
                    dst_ip_v4: "N/A".to_string(),
                    src_port: 0,
                    dst_port: 0,
                });
            }
        } else {
            eprintln!("Error parsing Ethernet packet");
        }
    }
    Ok(packets)
}

fn process_ipv4_packet(packets: &mut Vec<PacketInfo>, eth_packet: &EthernetPacket) {
    if let Some(ip_packet) = Ipv4Packet::new(eth_packet.payload()) {
        let src_ip_v4 = ip_packet.get_source().to_string();
        let dst_ip_v4 = ip_packet.get_destination().to_string();

        // FIXME: add port parsing in its own function for use with ipv6
        let (src_port, dst_port) = match ip_packet.get_next_level_protocol() {
            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                if let Some(tcp_packet) = TcpPacket::new(ip_packet.payload()) {
                    (tcp_packet.get_source(), tcp_packet.get_destination())
                } else {
                    (0, 0)
                }
            },
            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                if let Some(udp_packet) = UdpPacket::new(ip_packet.payload()) {
                    (udp_packet.get_source(), udp_packet.get_destination())
                } else {
                    (0, 0)
                }
            },
            _ => (0, 0),
        };

        packets.push(PacketInfo {
            date: Utc::now(),
            src_mac: eth_packet.get_source().to_string(),
            dst_mac: eth_packet.get_destination().to_string(),
            src_ip_v4,
            dst_ip_v4,
            src_port,
            dst_port,
        });
    }
}