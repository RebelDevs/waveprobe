use super::handlers::command_execute;
use regex::Regex;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::env;
use std::time::Duration;
use tokio::{task, time};

struct ConnectionSettings {
    uri: String,
    port: u16,
}

pub async fn init() {
    let (client, mut eventloop) = connect();
    subscribe_to_all(&client).await;

    // test
    let client_clone = client.clone();
    task::spawn(async move {
        for i in 0..10 {
            client_clone
                .publish(
                    "some_id/command/request",
                    QoS::AtLeastOnce,
                    false,
                    format!("hello {}", i),
                )
                .await
                .unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });
    // test end

    listen_to_events(&client, &mut eventloop).await;
}

async fn listen_to_events(_client: &AsyncClient, eventloop: &mut EventLoop) {
    loop {
        match eventloop.poll().await {
            Ok(notification) => match notification {
                Event::Incoming(Incoming::Publish(publish)) => {
                    let command_re = Regex::new(r"^(.*)/command/request$").unwrap();

                    if command_re.is_match(&publish.topic) {
                        match command_execute::handler::handle(&publish.payload) {
                            Ok(result) => {
                                println!("Result, {:#?}", result);
                            }
                            Err(e) => {
                                println!("err, {}", e);
                            }
                        }
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
