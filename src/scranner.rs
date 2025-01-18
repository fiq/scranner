use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use std::error::Error;

pub struct PacketInfo {
    pub src_ip: String,
    pub dst_ip: String,
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
        counter +=1;
        if let Some(eth_packet) = EthernetPacket::new(packet?) {
            println!("Captured Ethernet packet: {:?}", eth_packet);

            packets.push(PacketInfo {
                src_ip: eth_packet.get_source().to_string(),
                dst_ip: eth_packet.get_destination().to_string(),
                src_port: 0,
                dst_port: 0,
            });
        } else {
            eprintln!("Error parsing Ethernet packet");
        }
    }
    Ok(packets)
}
