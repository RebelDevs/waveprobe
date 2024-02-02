use super::handlers::command_execute;
use regex::Regex;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use serde::Serialize;
use serde_json;
use std::env;
use std::time::Duration;
use tokio::task;

struct ConnectionSettings {
    uri: String,
    port: u16,
}

pub async fn init() {
    let (mut client, mut eventloop) = connect();
    loop {
        subscribe_to_all(&client).await;

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

async fn publish_response<T: Serialize>(client: &AsyncClient, topic: String, data: T) {
    let response_replace_re = Regex::new(r"\/request$").unwrap();
    let response_topic = response_replace_re.replace(&topic, "/response").to_string();
    publish(client, response_topic, data).await;
}

async fn listen_to_events(client: &AsyncClient, eventloop: &mut EventLoop) -> Result<(), String> {
    loop {
        match eventloop.poll().await {
            Ok(notification) => match notification {
                Event::Incoming(Incoming::Publish(data)) => {
                    let command_re = Regex::new(r"^(.*)/command/request$").unwrap();

                    if command_re.is_match(&data.topic) {
                        let client_clone = client.clone();
                        task::spawn(async move {
                            match command_execute::handler::handle(&data.payload).await {
                                Ok(result) => {
                                    publish_response(&client_clone, data.topic, result).await
                                }
                                Err(e) => eprintln!("err, {}", e),
                            }
                        });
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
    client
        .subscribe("+/command/request", QoS::AtMostOnce)
        .await
        .unwrap();
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

    let mut mqttoptions = MqttOptions::new("some_id", env_options.uri, env_options.port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    AsyncClient::new(mqttoptions, 10)
}
