use std::sync::Mutex;
use std::{io::Write, net::TcpListener};

use rand::{thread_rng, Rng};
use std::time::Duration;

pub enum SensorState {
    Opened,
    Closed,
}

#[allow(async_fn_in_trait)]
pub trait Sensor {
    fn state(&self) -> SensorState;
}

pub struct SimpleSensor(Mutex<SensorState>);

impl SimpleSensor {
    pub fn new() -> Self {
        SimpleSensor(Mutex::new(SensorState::Opened))
    }
}

impl Sensor for SimpleSensor {
    fn state(&self) -> SensorState {
        let sleep_time = thread_rng().gen_range(0..10);
        std::thread::sleep(Duration::from_secs(sleep_time));
        let mut state_guard = self.0.lock().unwrap();
        match *state_guard {
            SensorState::Closed => {
                *state_guard = SensorState::Opened;
                SensorState::Opened
            }
            SensorState::Opened => {
                *state_guard = SensorState::Closed;
                SensorState::Closed
            }
        }
    }
}

pub struct TCPSensor(Mutex<SensorState>, TcpListener);

impl TCPSensor {
    pub fn new(connection_string: &str) -> Self {
        TCPSensor(
            Mutex::new(SensorState::Opened),
            TcpListener::bind(connection_string).expect("connection to be established correctly"),
        )
    }
}

impl Sensor for TCPSensor {
    fn state(&self) -> SensorState {
        let (mut conn, _) = self.1.accept().expect("Acception connection to work");
        let mut state_guard = self.0.lock().unwrap();

        // Construct the HTTP response with the appropriate Content-Length
        let body = match *state_guard {
            SensorState::Closed => "Sensor is closed",
            SensorState::Opened => "Sensor is opened",
        };
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        // Send the response
        conn.write_all(response.as_bytes())
            .expect("writing to the connection to work");

        // Flush the connection to ensure all data is sent
        conn.flush().expect("flushing the connection to work");

        conn.write_all(response.as_bytes())
            .expect("writing to the connection to work");

        match *state_guard {
            SensorState::Closed => {
                *state_guard = SensorState::Opened;
                SensorState::Opened
            }
            SensorState::Opened => {
                *state_guard = SensorState::Closed;
                SensorState::Closed
            }
        }
    }
}
