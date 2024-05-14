#[derive(Clone)]
pub struct Occupancy(pub usize, pub usize);

pub trait Controller {
    fn get_occupancy(&mut self) -> Occupancy;
}

pub mod multicontroller {
    use super::*;
    use crate::sensor::{Sensor, SensorState};
    use std::{
        sync::{
            mpsc::{self, Receiver, Sender},
            Arc, Mutex,
        },
        thread::{self, JoinHandle},
    };

    pub struct MultiController {
        occupancy: Occupancy,
        receiver: Receiver<SensorState>,
        sender: Sender<SensorState>,
        threads: Vec<JoinHandle<()>>,
    }

    impl Controller for MultiController {
        fn get_occupancy(&mut self) -> Occupancy {
            let ss = self.receiver.recv().unwrap();
            match ss {
                SensorState::Closed => self.occupancy.0 += 1,
                SensorState::Opened => self.occupancy.0 -= 1,
            };
            self.occupancy.clone()
        }
    }

    impl MultiController {
        pub fn new() -> Self {
            let (sx, rx) = mpsc::channel::<SensorState>();
            MultiController {
                occupancy: Occupancy(0, 0),
                receiver: rx,
                sender: sx,
                threads: Vec::new(),
            }
        }
        pub fn add_sensor<S: Sensor + Send + 'static>(&mut self, sensor: S) {
            let wrapped = Arc::new(Mutex::new(sensor));
            let sender_clone = self.sender.clone();
            let thread_handle = thread::spawn(move || loop {
                let sensor = wrapped.lock().unwrap();
                sender_clone.send(sensor.state()).unwrap();
            });
            self.threads.push(thread_handle);
            self.occupancy.1 += 1;
        }
    }
}
