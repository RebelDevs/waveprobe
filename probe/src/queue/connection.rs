use super::super::commands;
use super::handlers::command_execute;
use regex::Regex;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, Publish, QoS};
use serde::Serialize;
use serde_json;
use std::env;
use std::time::Duration;
use tokio::{task, time};

struct ConnectionSettings {
    uri: String,
    port: u16,
}

fn execute_test_queues(client: &AsyncClient) {
    let client_clone = client.clone();
    task::spawn(async move {
        publish_test_queues(&client_clone).await;
    });
}

#[async_recursion]
async fn publish_test_queues(client: &AsyncClient) {
    for _ in 0..1 {
        let options = commands::ping::ping::Options {
            hostname: "google.com".to_string(),
            packets: 4,
        };
        let command = command_execute::handler::CommandRequest {
            command: String::from("ping"),
            id: String::from("123"),
            options,
        };

        publish(&client, "some_id/command/request".to_string(), command).await;
        time::sleep(Duration::from_millis(100)).await;
    }

    time::sleep(Duration::from_millis(10000)).await;
    publish_test_queues(client).await;
}

pub async fn init() {
    let (client, mut eventloop) = connect();
    subscribe_to_all(&client).await;

    // test
    execute_test_queues(&client);
    // test end

    listen_to_events(&client, &mut eventloop).await;
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
    publish(&client, response_topic, data).await;
}

async fn listen_to_events(client: &AsyncClient, eventloop: &mut EventLoop) {
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
                    } else {
                        println!(
                            "Received = {}",
                            String::from_utf8(publish.payload.to_vec()).unwrap()
                        );
                    }
                }
                // Handle other events...
                _ => {
                    println!("Other = {:?}", notification);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                break; // Or handle the error (e.g., retry connection)
            }
        }
    }
}

async fn subscribe_to_all(client: &AsyncClient) {
    client
        .subscribe("some_id/command/request", QoS::AtMostOnce)
        .await
        .unwrap();
}

fn get_connection_data() -> ConnectionSettings {
    ConnectionSettings {
        uri: env::var("MQTT_URI").map_or("broker.emqx.io".to_string(), |x| x),
        port: env::var("MQTT_PORT")
            .ok()
            .and_then(|x| x.parse::<u16>().ok())
            .unwrap_or(1883),
    }
}

fn connect() -> (AsyncClient, EventLoop) {
    let env_options = get_connection_data();

    let mut mqttoptions = MqttOptions::new("some_id", env_options.uri, env_options.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    AsyncClient::new(mqttoptions, 10)
}
