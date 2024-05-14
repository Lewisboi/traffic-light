use crate::controller::{Controller, Occupancy};
use crate::traffic_light::TrafficLight;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Orchestrator<T: TrafficLight, C: Controller> {
    controller: Arc<Mutex<C>>,
    traffic_light: T,
}

impl<T: TrafficLight, C: Controller> Orchestrator<T, C> {
    pub fn new(ctrl: C, tl: T) -> Self {
        Self {
            controller: Arc::new(Mutex::new(ctrl)),
            traffic_light: tl,
        }
    }
}

impl<T: TrafficLight, C: Controller + Send + 'static> Orchestrator<T, C> {
    pub fn run(&mut self) {
        let (s, r) = mpsc::channel::<Occupancy>();
        let s_clone = s.clone();
        let c_clone = self.controller.clone();
        thread::spawn(move || loop {
            let occ = c_clone.lock().unwrap().get_occupancy();
            s_clone.send(occ).unwrap();
        });

        for event in r.iter() {
            self.traffic_light.update(event);
        }
    }
}
