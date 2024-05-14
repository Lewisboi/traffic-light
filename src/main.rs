use crate::controller::multicontroller::MultiController;
use crate::orchestrator::Orchestrator;
use crate::sensor::{SimpleSensor, TCPSensor};
use crate::traffic_light::SimpleTrafficLight;

const SENSOR_COUNT: u8 = 3;

fn main() {
    let mut controller = MultiController::new();
    for i in 0..(SENSOR_COUNT - 1) {
        let connection_string = format!("127.0.0.1:808{i}");
        controller.add_sensor(TCPSensor::new(&connection_string));
    }
    controller.add_sensor(SimpleSensor::new());

    let mut orchestrator = Orchestrator::new(controller, SimpleTrafficLight::new());
    orchestrator.run();
}

pub mod sensor;

pub mod traffic_light;

pub mod orchestrator;

pub mod controller;
