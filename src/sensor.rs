use std::os::macos::raw::stat;

use tokio::sync::Mutex;

use rand::{thread_rng, Rng};
use tokio::time::{self, Duration};

pub enum SensorEvent {
    Open,
    Close,
}

#[allow(async_fn_in_trait)]
pub trait Sensor {
    async fn sense(&mut self) -> SensorEvent;
}

enum SensorState {
    Opened,
    Closed,
}
pub struct SimpleSensor(Mutex<SensorState>);

impl SimpleSensor {
    pub fn new() -> Self {
        SimpleSensor(Mutex::new(SensorState::Opened))
    }
}

impl Sensor for SimpleSensor {
    async fn sense(&mut self) -> SensorEvent {
        let sleep_time = thread_rng().gen_range(0..10);
        time::sleep(Duration::from_secs(sleep_time)).await;
        let mut state_guard = self.0.lock().await;
        match *state_guard {
            SensorState::Closed => {
                *state_guard = SensorState::Opened;
                SensorEvent::Open
            }
            SensorState::Opened => {
                *state_guard = SensorState::Closed;
                SensorEvent::Close
            }
        }
    }
}
