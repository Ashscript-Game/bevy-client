use bevy::{prelude::*, utils::hashbrown::HashMap};
use hexx::hex;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::{hash::Hash, time::Duration};

use crate::components::{Actions, State};

pub fn setup_receiver(mut state: ResMut<State>, mut actions: ResMut<Actions>) {

    let mut value: HashMap<String, String> = HashMap::new();

    let callback = move |payload: &Payload, socket: &RawClient, value: &mut HashMap<String, String>/* , state: &mut  State*/| {
        match payload {
            Payload::String(str) => {
                println!("Received: {}", str);
                value.insert("key".to_string(), "value".to_string());
                /* state.map = serde_json::from_str(&str).unwrap(); */
            },
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
            Payload::Text(text) => {
                println!("Received text: {:#?}", text);
            },
        }
    };

    // get a socket that is connected to the admin namespace
    let socket = ClientBuilder::new("http://localhost:3000")
        .namespace("/client")
        .on("keyframe", move |payload: Payload, socket: RawClient| {
            
            // callback(&payload, &socket, &mut value);
            value.insert("key".to_string(), "value".to_string());

            match payload {
                Payload::String(str) => {
                    println!("Received: {}", str);
                    value.insert("key".to_string(), "value".to_string());
                    /* state.map = serde_json::from_str(&str).unwrap(); */
                    
                    // state.map.chunk_at(&hex(0, 0));
                },
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                Payload::Text(text) => {
                    println!("Received text: {:#?}", text);
                },
            }

            socket
            .emit("test", json!({"this is an ack": true}))
            .expect("Server unreachable")
        })
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
