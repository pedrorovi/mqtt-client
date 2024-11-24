use esp_idf_svc::mqtt::client::QoS;
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_sys::EspError;

pub struct MqttClient<'a> {
    client: EspMqttClient<'a>,
}

impl<'a> MqttClient<'a> {
    pub fn new(url: &str, port: u16, id: &str, user: &str, pass: &str) -> Result<Self, EspError> {
        let mut config = MqttClientConfiguration::default();
        let uri = format!("mqtt://{}:{}", url, port);
        config.client_id = Some(id);
        config.username = Some(user);
        config.password = Some(pass);
        match EspMqttClient::new(uri.as_str(), &config) {
            Ok((client, _connection)) => Ok(MqttClient { client }),
            Err(e) => {
                eprintln!("Failed to create MQTT client: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn publish(&mut self, topic: &str, payload: &str, qos: u8) -> Result<(), EspError> {
        let qos = match qos {
            0 => QoS::AtMostOnce,
            1 => QoS::AtLeastOnce,
            2 => QoS::ExactlyOnce,
            _ => QoS::AtMostOnce,
        };
        self.client
            .publish(topic, qos, true, payload.as_bytes())
            .map(|_| ())
    }
}
