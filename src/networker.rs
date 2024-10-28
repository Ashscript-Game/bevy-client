use ashscript_types::{keyframe::KeyFrame, map::Map};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::{hash::Hash, time::Duration};

use crate::components::{Actions, State};

pub fn setup_receiver(mut state: ResMut<State>, mut actions: ResMut<Actions>) {

    let mut value: HashMap<String, String> = HashMap::new();

    let callback = move |payload: &Payload, socket: &RawClient, value: &mut HashMap<String, String>/* , state: &mut  State*/| {
        match payload {
            Payload::String(str) => {
                println!("Received string: {}", str);
                value.insert("key".to_string(), "value".to_string());
                /* state.map = serde_json::from_str(&str).unwrap(); */
            },
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
            Payload::Text(text) => {
                println!("Received text: {:#?}", text);

                // let res = serde_json::from_str(&text).expect("unable to deserialize");
                // serde_json::from_value(value)
                let ser_keyframe = text.iter().filter_map(|v| {
                    match v {
                        serde_json::Value::Bool(z) => {
                            println!("Received bool: {:#?}", z);
                            None
                        },
                        serde_json::Value::Number(n) => {
                            println!("Received number: {:#?}", n);

                            Some(n.as_u64().unwrap() as u8)
                        },
                        serde_json::Value::String(s) => {
                            println!("Received string: {:#?}", s);
                            None
                        },
                        _ => {
                            println!("Received unknown: {:#?}", v);
                            None
                        },
                    }
                }).collect::<Vec<u8>>();

                let keyframe = postcard::from_bytes::<KeyFrame>(ser_keyframe.as_slice()).expect("unable to deserialize");

                println!("processed keyframe for tick: {}", keyframe.global.tick);
            },
        }

        socket
            .emit("test", json!({"this is an ack": true}))
            .expect("Server unreachable")
    };
    
    // get a socket that is connected to the admin namespace
    let socket = ClientBuilder::new("http://localhost:3000")
        .namespace("/client")
        .on("game_state", move |payload: Payload, socket: RawClient| callback(&payload, &socket, &mut value))
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");
}

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