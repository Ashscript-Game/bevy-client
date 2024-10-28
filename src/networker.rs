use std::time::Duration;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_mqtt::{rumqttc::{MqttOptions, QoS, Transport}, MqttClient, MqttClientConnected, MqttClientError, MqttConnectError, MqttEvent, MqttSetting, SubscribeTopic, TopicMessage};

pub fn setup_clients(mut commands: Commands) {
    commands.spawn(MqttSetting {
        mqtt_options: MqttOptions::new("bevy-mqtt-client", "http://localhost", 3000),
        cap: 10,
    });

    // spawn websocket client
    let mut mqtt_options = MqttOptions::new("mqtt-ws-client", "ws://127.0.0.1:8080", 8080);
    mqtt_options.set_transport(Transport::Ws);
    // mqtt_options.set_credentials("username", "password");
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    commands.spawn((
        MqttSetting {
            mqtt_options,
            cap: 10,
        },
        WebsocketMqttClient,
    ));
}

// just a marker component for filter
#[derive(Component)]
struct WebsocketMqttClient;

/// this is a system that subscribes to a topic and handle the incoming messages
pub fn sub_topic(
    mqtt_client: Query<(Entity, &MqttClient, &MqttSetting), Added<MqttClientConnected>>,
    mut commands: Commands,
) {
    for (entity, client, setting) in mqtt_client.iter() {
        client
            .subscribe("hello".to_string(), QoS::AtMostOnce)
            .unwrap();

        let setting = setting.clone();
        let child_id = commands
            .spawn(SubscribeTopic::new("/client", QoS::AtMostOnce))
            .observe(move |topic_message: Trigger<TopicMessage>| {
                println!(
                    "{:?}: Topic: '+/mqtt' received : {:?}",
                    setting.mqtt_options.broker_address().clone(),
                    topic_message.event().payload
                );
            })
            .id();
        commands.entity(entity).add_child(child_id);
    }
}

/// this is global handler for all incoming messages
pub fn handle_message(mut mqtt_event: EventReader<MqttEvent>) {
    for event in mqtt_event.read() {
        match &event.event {
            rumqttc::Event::Incoming(income) => match income {
                rumqttc::Incoming::Publish(publish) => {
                    println!(
                        "Topic Component: {} Received: {:?}",
                        publish.topic, publish.payload
                    );
                }
                _ => {
                    println!("Incoming: {:?}", income);
                }
            },
            rumqttc::Event::Outgoing(_) => {}
        }
    }
}

pub fn handle_error(
    mut connect_errors: EventReader<MqttConnectError>,
    mut client_errors: EventReader<MqttClientError>,
) {
    for error in connect_errors.read() {
        println!("connect Error: {:?}", error);
    }

    for error in client_errors.read() {
        println!("client Error: {:?}", error);
    }
}

pub fn publish_message(mqtt_client: Query<&MqttClient, With<MqttClientConnected>>) {
    for client in mqtt_client.iter() {
        client
            .publish(
                "client".to_string(),
                QoS::AtMostOnce,
                false,
                "mqtt".as_bytes(),
            )
            .unwrap();
        for i in 0..3 {
            client
                .publish(format!("{}/mqtt", i), QoS::AtMostOnce, false, b"hello")
                .unwrap();
        }
    }
}