use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    Packet,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use tauri::Window;

pub struct PacketSniffer {
    should_stop: Arc<AtomicBool>,
    event_sender: Option<Sender<String>>,
    sniffing_thread: Option<thread::JoinHandle<()>>,
    event_thread: Option<thread::JoinHandle<()>>,
}
impl PacketSniffer {
    pub fn new() -> Self {
        Self {
            should_stop: Arc::new(AtomicBool::new(false)),
            event_sender: None,
            sniffing_thread: None,
            event_thread: None,
        }
    }

    pub fn start(&mut self, interface_name: String, window: Window) {
        let should_stop = self.should_stop.clone();

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        self.event_sender = Some(tx);

        let sniffing_thread = thread::spawn(move || {
            let interfaces = datalink::interfaces();
            let interface = interfaces
                .into_iter()
                .filter(|iface: &NetworkInterface| iface.name == interface_name)
                .next()
                .expect("Failed to get interface");

            // Create a channel to receive packets from the network interface
            let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
                Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type\n"),
                Err(e) => panic!("Error creating channel: {}\n", e),
            };

            while !should_stop.load(Ordering::SeqCst) {
                match rx.next() {
                    Ok(packet) => {
                        let packet = EthernetPacket::new(packet).unwrap();
                        handle_packet(&packet);
                    }
                    Err(e) => {
                        eprintln!("An error occurred while reading: {}", e);
                    }
                }
            }
        });

        // Thread to listen for packets and emit events back to Tauri
        let event_thread = thread::spawn(move || {
            for packet in rx {
                // Passing information to the Tauri window's webview to update frontend
                window
                    .emit("packet", &packet)
                    .expect("failed to emit event");
            }
        });

        self.sniffing_thread = Some(sniffing_thread);
        self.event_thread = Some(event_thread);
    }

    pub fn stop(&mut self) {
        self.should_stop.store(true, Ordering::SeqCst);

        // Join the sniffing thread
        if let Some(thread) = self.sniffing_thread.take() {
            thread.join().expect("Failed to join sniffing thread");
        }

        // Close the channel to stop the event thread
        self.event_sender.take();

        // Join the event emitter thread
        if let Some(thread) = self.event_thread.take() {
            thread.join().expect("Failed to join event thread");
        }
    }
}

// Packet handling logic similar to what you've already written.
fn handle_packet(ethernet: &EthernetPacket) {
    // Print the entire Ethernet frame
    // println!("Got an ethernet packet: {:?}", ethernet);
    // Determine the type of packet encapsulated within the Ethernet frame
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
    }
}
