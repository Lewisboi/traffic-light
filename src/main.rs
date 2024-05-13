use crate::orchestrator::Orchestrator;
use crate::sensor::SimpleSensor;
use crate::traffic_light::SimpleTrafficLight;

const SENSOR_COUNT: u8 = 3;

#[tokio::main]
async fn main() {
    let mut orchestrator = Orchestrator::new(SimpleTrafficLight::new(SENSOR_COUNT));
    for _ in 0..SENSOR_COUNT {
        orchestrator.add_sensor(SimpleSensor::new());
    }
    orchestrator.run().await
}

pub mod sensor;

pub mod traffic_light;

pub mod orchestrator;
