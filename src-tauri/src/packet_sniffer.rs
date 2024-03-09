use pnet::datalink;
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    Packet,
};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn get_network_interfaces() -> Vec<String> {
    datalink::interfaces()
        .into_iter()
        .map(|interface| interface.name)
        .collect::<Vec<String>>()
}

#[tauri::command]
pub fn log_packets(interface_name: String, app_handle: AppHandle) -> Result<(), String> {
    // Get all interfaces
    let interfaces = datalink::interfaces();
    // Fitler the list to find the given interface name
    let interface = match interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
    {
        Some(interface) => interface,
        None => return Err(format!("Interface {} not found", interface_name)),
    };

    sudo::escalate_if_needed().unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => return Err(format!("Unhandled channel type")),
        Err(e) => {
            return Err(format!(
                "An error occured when creating the datalink channel: {}",
                e
            ))
        }
    };

    // Spawn a new thread to process packets without blocking the UI
    std::thread::spawn(move || {
        // Loop over the packets 10 times
        for _ in 0..10 {
            match rx.next() {
                Ok(packet) => {
                    let packet = EthernetPacket::new(packet).unwrap();
                    parse_packet(&packet, &app_handle);
                }
                Err(e) => {
                    // Print the error
                    eprintln!("An error occured while reading: {}", e);
                    return;
                }
            }
        }
    });
    Ok(())
}

fn parse_packet(ethernet: &EthernetPacket, app_handle: &AppHandle) -> () {
    match ethernet.get_ethertype() {
        // If it's an IPv4 packet, handle it
        EtherTypes::Ipv4 => {
            // println!("IPv4 packet");
            // Attempt to construct an IPv4 packet from the Ethernet payload
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                // Print the IPv4 header
                // println!("Header: {:?}", header);
                // Check the protocol of the encapsulated data
                match header.get_next_level_protocol() {
                    // If it's a TCP packet, handle it
                    IpNextHeaderProtocols::Tcp => {
                        // println!("TCP packet");
                        // Attempt to construct a TCP packet from the IPv4 payload
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            // Print the TCP packet
                            // println!("TCP packet: {:?}", tcp);
                            // Print source and destination IPs and ports
                            println!(
                                "Got a TCP packet from {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                            app_handle
                                .emit_all(
                                    "packet",
                                    format!(
                                        "Got a TCP packet from {}:{} to {}:{}",
                                        header.get_source(),
                                        tcp.get_source(),
                                        header.get_destination(),
                                        tcp.get_destination()
                                    ),
                                )
                                .expect("Failed to emit packet event");
                        } else {
                            // The TCP packet couldn't be parsed
                            println!("Malformed TCP packet");
                        }
                    }
                    // Ignore packets that are not TCP
                    _ => {
                        println!("Ignoring non-TCP packet");
                    }
                }
            } else {
                // The IPv4 packet couldn't be parsed
                println!("Malformed IPv4 packet");
            }
        }
        // Ignore packets that are not IPv4
        _ => {
            println!("Ignoring non-IPv4 packet");
        }
    };
}
