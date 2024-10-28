use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_simple_networking::{NetworkEvent, SocketAddrResource, Transport};
use hexx::hex;
use serde_json::json;

pub fn run_network(mut events: EventReader<NetworkEvent>) {
    for event in events.read() {
        match event {
            NetworkEvent::Message(_, message) => {
                info!("Message received: {}", String::from_utf8_lossy(message));
            }
            NetworkEvent::SendError(err, msg) => {
                error!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            }
            NetworkEvent::RecvError(err) => {
                error!("NetworkEvent::RecvError: {:?}", err);
            }
            // discard irrelevant events
            _ => {}
        }
    }
}

pub fn send_message(address: Res<SocketAddrResource>, mut transport: ResMut<Transport>) {
    transport.send(**address, b"Hello world!")
}