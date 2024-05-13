use crate::orchestrator::Orchestrator;
use crate::sensor::TCPSensor;
use crate::traffic_light::SimpleTrafficLight;

const SENSOR_COUNT: u8 = 3;

#[tokio::main]
async fn main() {
    let mut orchestrator = Orchestrator::new(SimpleTrafficLight::new(SENSOR_COUNT));
    for i in 0..SENSOR_COUNT {
        let connection_string = format!("127.0.0.1:808{i}");
        orchestrator.add_sensor(TCPSensor::new(&connection_string).await);
    }
    orchestrator.run().await
}

pub mod sensor;

pub mod traffic_light;

pub mod orchestrator;
