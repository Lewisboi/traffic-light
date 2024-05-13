use tokio::sync::Mutex;
use tokio::{io::AsyncWriteExt, net::TcpListener};

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

pub struct TCPSensor(Mutex<SensorState>, TcpListener);

impl TCPSensor {
    pub async fn new(connection_string: &str) -> Self {
        TCPSensor(
            Mutex::new(SensorState::Opened),
            TcpListener::bind(connection_string)
                .await
                .expect("connection to be established correctly"),
        )
    }
}

impl Sensor for TCPSensor {
    async fn sense(&mut self) -> SensorEvent {
        let (mut conn, _) = self.1.accept().await.expect("Acception connection to work");
        let mut state_guard = self.0.lock().await;

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
            .await
            .expect("writing to the connection to work");

        // Flush the connection to ensure all data is sent
        conn.flush().await.expect("flushing the connection to work");

        conn.write_all(response.as_bytes())
            .await
            .expect("writing to the connection to work");

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
