use crate::orchestrator::Orchestrator;
use crate::sensor::{SimpleSensor, TCPSensor};
use crate::traffic_light::SimpleTrafficLight;

const SENSOR_COUNT: u8 = 3;

fn main() {
    let mut orchestrator = Orchestrator::new(SimpleTrafficLight::new(SENSOR_COUNT));
    for i in 0..(SENSOR_COUNT - 1) {
        let connection_string = format!("127.0.0.1:808{i}");
        orchestrator.add_sensor(TCPSensor::new(&connection_string));
    }
    orchestrator.add_sensor(SimpleSensor::new());
    orchestrator.run();
}

pub mod sensor;

pub mod traffic_light;

pub mod orchestrator;
