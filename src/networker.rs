use crate::components::{Actions, State};
use ashscript_types::keyframe::KeyFrame;
use bevy::{
    app::{App, Plugin, Startup},
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
    utils::hashbrown::HashMap,
};
use bevy::{prelude::*, render::settings, tasks::TaskPool};
use bevy_eventwork::{EventworkRuntime, Network};
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::{net::TcpStream, sync::mpsc::Receiver};

/*
pub fn setup_receiver(mut state: ResMut<State> /* , mut actions: ResMut<Actions> */) {
    let mut value: HashMap<String, String> = HashMap::new();

    let callback = afunc(
        move |payload: Payload,
              socket: RawClient /* value: &mut HashMap<String, String> */| {
            let mut state = state;
            match payload {
                Payload::String(str) => {
                    println!("Received string: {}", str);
                    value.insert("key".to_string(), "value".to_string());
                }
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                Payload::Text(text) => {
                    println!("Received text: {:#?}", text);

                    // let res = serde_json::from_str(&text).expect("unable to deserialize");
                    // serde_json::from_value(value)
                    let ser_keyframe = text
                        .iter()
                        .filter_map(|v| match v {
                            serde_json::Value::Bool(z) => {
                                println!("Received bool: {:#?}", z);
                                None
                            }
                            serde_json::Value::Number(n) => {
                                println!("Received number: {:#?}", n);

                                Some(n.as_u64().unwrap() as u8)
                            }
                            serde_json::Value::String(s) => {
                                println!("Received string: {:#?}", s);
                                None
                            }
                            _ => {
                                println!("Received unknown: {:#?}", v);
                                None
                            }
                        })
                        .collect::<Vec<u8>>();

                    let keyframe = postcard::from_bytes::<KeyFrame>(ser_keyframe.as_slice())
                        .expect("unable to deserialize");
                    // state.map = keyframe.map;

                    println!("processed keyframe for tick: {}", keyframe.global.tick);
                }
            }

            socket
                .emit("test", json!({"this is an ack": true}))
                .expect("Server unreachable")
        },
    );

    // get a socket that is connected to the admin namespace
    let socket = ClientBuilder::new("http://localhost:3000")
        .namespace("/client")
        .on("game_state", callback)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");
}
*/

/* fn state_callback<T: Into<Event>, F>(payload: Payload, client: Client, state: &ResMut<State>) -> Self
where
    F: for<'a> std::ops::FnMut(Payload, Client) -> BoxFuture<'static, ()>
        + 'static
        + Send
        + Sync, {
    println!("received keyframe: {:?}", payload);

    async move {
        client
        .emit("test", json!({"got ack": true}))
        .await
        .expect("Server unreachable");
    }
    .boxed()
} */

/* fn actions_callback(payload: Payload, client: Client) {
    println!("received action: {:?}", payload);
} */

// startup
// create a receiver
// when it receives emissions

// update

/* pub fn connection_handler(mut events: EventReader<NetworkEvent>) {
    for event in events.read() {
        match event {
            NetworkEvent::Message(_, msg) => {
                info!("{}", String::from_utf8_lossy(msg));
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

pub fn hello_world(remote_addr: Res<SocketAddrResource>, mut transport: ResMut<Transport>) {
    transport.send(**remote_addr, b"Hello world!");
} */

#[derive(Resource)]
pub struct NetworkInfo {
    // This isn't used right now but seems to be used internally by `ewebsock`
    // to track whether or not to keep the connection open, so let's just keep
    // it here.
    pub sender: std::sync::Mutex<ewebsock::WsSender>,
    pub receiver: std::sync::Mutex<ewebsock::WsReceiver>,
}

pub fn create_network_resource() -> NetworkInfo {
    let options = ewebsock::Options::default();
    let (sender, receiver) = ewebsock::connect("ws://localhost:3000/game-state", options).unwrap();

    let _ = sender;

    return NetworkInfo {
        sender: std::sync::Mutex::new(sender),
        receiver: std::sync::Mutex::new(receiver),
    };
}

pub fn setup_receiver(
    net: ResMut<Network<WebSocketProvider>>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
    settings: Res<NetworkSettings>,
) {
    net.connect(
        url::Url::parse("ws://localhost:3000/game-state").unwrap(),
        &task_pool.0,
        &settings,
    );
}

pub fn handle_network_events(network_info: ResMut<NetworkInfo>) {
    if let Some(message) = network_info.receiver.lock().unwrap().try_recv() {
        info!("Received event");
        match message {
            ewebsock::WsEvent::Opened => {
                println!("connected");
            }

            ewebsock::WsEvent::Closed => {
                println!("disconnected");
            }
            ewebsock::WsEvent::Message(ewebsock::WsMessage::Binary(data)) => {
                let keyframe: KeyFrame =
                    postcard::from_bytes(&data).expect("failed to deserialize keyframe");
                println!("{:?}", keyframe);
            }
            ewebsock::WsEvent::Message(msg) => {
                println!("received message {:?}", msg);
            }

            ewebsock::WsEvent::Error(err) => {
                println!("recv error: {:?}", err);
            }
        }
    }
}