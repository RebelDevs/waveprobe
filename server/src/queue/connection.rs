use super::handlers;
use crate::commands;
use regex::Regex;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use serde::Serialize;
use serde_json::{self};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

struct ConnectionSettings {
    uri: String,
    port: u16,
}

pub async fn init() -> AsyncClient {
    let (client, eventloop) = connect();
    subscribe_to_all(&client).await;

    // TODO: remove test publish
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

    let client_clone = client.clone();
    tokio::spawn(async move {
        let el_mutex = Arc::new(Mutex::new(eventloop));
        loop {
            poll_events(&client_clone, el_mutex.clone()).await;
        }
    });

    return client;
}

pub async fn publish<T: Serialize>(client: &AsyncClient, topic: String, data: T) {
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

async fn poll_events(_client: &AsyncClient, el_mutex: Arc<Mutex<EventLoop>>) {
    let mut el = el_mutex.lock().await;
    match el.poll().await {
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
        }
    };
}

async fn subscribe_to_all(client: &AsyncClient) {
    let ack = client.subscribe("+/command/ack", QoS::AtMostOnce);
    let response = client.subscribe(handlers::cmd_resp::SUB_NAME, QoS::AtMostOnce);

    let (ack_output, resp_output) = tokio::join!(ack, response);

    match ack_output {
        Ok(_) => {}
        Err(e) => {
            eprintln!("err: {}", e);
        }
    }

    match resp_output {
        Ok(_) => {}
        Err(e) => {
            eprintln!("err: {}", e);
        }
    }
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
