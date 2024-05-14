use crate::sensor::{Sensor, SensorEvent, TCPSensor};
use crate::traffic_light::TrafficLight;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Orchestrator<T: TrafficLight> {
    sensors: Vec<Arc<Mutex<dyn Sensor + Send>>>,
    traffic_light: T,
}

impl<T: TrafficLight> Orchestrator<T> {
    pub fn new(tl: T) -> Self {
        Self {
            sensors: Vec::new(),
            traffic_light: tl,
        }
    }

    pub fn add_sensor(&mut self, sensor: TCPSensor) {
        self.sensors.push(Arc::new(Mutex::new(sensor)));
    }
}

impl<T: TrafficLight> Orchestrator<T> {
    pub fn run(&mut self) {
        let (s, r) = mpsc::channel::<SensorEvent>();

        for sensor in &self.sensors {
            let sensor = sensor.clone();
            let sender = s.clone();
            thread::spawn(move || loop {
                let mut mutex_guard = sensor.lock().unwrap();
                let event = mutex_guard.sense();
                sender.send(event).unwrap();
            });
        }

        for event in r.iter() {
            self.traffic_light.update(event);
        }
    }
}
