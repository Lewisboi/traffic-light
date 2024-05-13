use crate::sensor::Sensor;
use crate::sensor::SensorEvent;
use crate::sensor::SimpleSensor;
use crate::traffic_light::TrafficLight;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct Orchestrator<T: TrafficLight> {
    sensors: Vec<Arc<Mutex<SimpleSensor>>>,
    traffic_light: Mutex<T>,
}

impl<T: TrafficLight> Orchestrator<T> {
    pub fn new(tl: T) -> Self {
        Self {
            sensors: Vec::new(),
            traffic_light: Mutex::new(tl),
        }
    }

    pub fn add_sensor(&mut self, sensor: SimpleSensor) {
        self.sensors.push(Arc::new(Mutex::new(sensor)));
    }
}

impl<T: TrafficLight> Orchestrator<T> {
    pub async fn run(&self) {
        let (s, mut r) = mpsc::channel::<SensorEvent>(32);

        for sensor in &self.sensors {
            let sensor = sensor.clone();
            let sender = s.clone();
            tokio::spawn(async move {
                loop {
                    let mut mutex_guard = sensor.lock().await;
                    let event = mutex_guard.sense().await;
                    sender.send(event).await.unwrap();
                }
            });
        }

        while let Some(event) = r.recv().await {
            let mut light_guard = self.traffic_light.lock().await;
            light_guard.update(event).await;
        }
    }
}
