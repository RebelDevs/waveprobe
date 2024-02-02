use crate::commands;
use super::handlers;
use regex::Regex;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use serde::Serialize;
use serde_json::{self};
use std::env;
use std::time::Duration;

struct ConnectionSettings {
    uri: String,
    port: u16,
}

pub async fn init() {
    let (mut client, mut eventloop) = connect();
    loop {
        subscribe_to_all(&client).await;

        let options = commands::ping::ping::Options {
            hostname: "google.com".to_string(),
            packets: 4,
        };
        let command = commands::exec::CommandRequest {
            command: String::from("ping"),
            id: String::from("123"),
            options,
        };

        publish(&client, "uk/command/request".to_string(), command).await;

        match listen_to_events(&client, &mut eventloop).await {
            Ok(_) => break,
            Err(_) => {
                let (new_client, new_eventloop) = connect();
                client = new_client;
                eventloop = new_eventloop;
            }
        }
    }
}

async fn publish<T: Serialize>(client: &AsyncClient, topic: String, data: T) {
    let result_bytes = serde_json::to_string(&data).unwrap();
    let publish_result = client
        .publish(topic.clone(), QoS::AtLeastOnce, false, result_bytes)
        .await;

    match publish_result {
        Ok(_) => {
            println!("Published [{}]", topic);
        }
        Err(e) => {
            eprintln!("Failed to publish, {}", e);
        }
    };
}

async fn listen_to_events(_client: &AsyncClient, eventloop: &mut EventLoop) -> Result<(), String> {
    loop {
        match eventloop.poll().await {
            Ok(notification) => match notification {
                Event::Incoming(Incoming::Publish(data)) => {
                    let command_ack_re = Regex::new(r"^(.*)/command/ack$").unwrap();

                    if command_ack_re.is_match(&data.topic) {
                        println!("command ack");
                    } else if handlers::cmd_resp::is_match(&data.topic) {
                        let _ = handlers::cmd_resp::handle(&data.payload);
                    }
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(format!("event loop error, {}", e));
            }
        }
    }
}

async fn subscribe_to_all(client: &AsyncClient) {
    let ack = client.subscribe("+/command/ack", QoS::AtMostOnce);
    let response = client.subscribe("+/command/response", QoS::AtMostOnce);

    tokio::join!(ack, response);
}

fn get_connection_data() -> ConnectionSettings {
    ConnectionSettings {
        uri: env::var("MQTT_URI").map_or("0.0.0.0".to_string(), |x| x),
        port: env::var("MQTT_PORT")
            .ok()
            .and_then(|x| x.parse::<u16>().ok())
            .unwrap_or(1883),
    }
}

fn connect() -> (AsyncClient, EventLoop) {
    let env_options = get_connection_data();

    let mut mqttoptions = MqttOptions::new("server", env_options.uri, env_options.port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    AsyncClient::new(mqttoptions, 10)
}
