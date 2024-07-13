use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};

pub async fn handle(client: &AsyncClient, request_id: String, payload: &[8]) {
}
