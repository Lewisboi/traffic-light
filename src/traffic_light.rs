use crate::sensor::{self, SensorEvent};

#[derive(PartialEq, Debug)]
pub enum TrafficLightState {
    Green,
    Red,
}

#[allow(async_fn_in_trait)]
pub trait TrafficLight {
    fn update(&mut self, status: sensor::SensorEvent);
}

pub struct SimpleTrafficLight {
    state: TrafficLightState,
    sensor_amount: u8,
    currently_closed: u8,
}

impl SimpleTrafficLight {
    pub fn new(sensor_amount: u8) -> Self {
        SimpleTrafficLight {
            state: TrafficLightState::Green,
            sensor_amount,
            currently_closed: 0,
        }
    }
}

impl ToString for SimpleTrafficLight {
    fn to_string(&self) -> String {
        let tl_widget = match self.state {
            TrafficLightState::Green => String::from("| |\n|G|"),
            TrafficLightState::Red => String::from("|R|\n| |"),
        };
        format!(
            "{}\n{}/{}",
            tl_widget, self.currently_closed, self.sensor_amount
        )
    }
}

impl TrafficLight for SimpleTrafficLight {
    fn update(&mut self, status: sensor::SensorEvent) {
        match status {
            SensorEvent::Open => {
                self.currently_closed -= 1;
                self.state = TrafficLightState::Green;
            }
            SensorEvent::Close => {
                self.currently_closed += 1;
                if self.currently_closed == self.sensor_amount {
                    self.state = TrafficLightState::Red;
                }
            }
        }
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simple_traffic_light_updates_correctly() {
        let mut tl = SimpleTrafficLight::new(3);
        tl.update(SensorEvent::Close);
        assert_eq!(tl.state, TrafficLightState::Green);
        tl.update(SensorEvent::Close);
        assert_eq!(tl.state, TrafficLightState::Green);
        tl.update(SensorEvent::Close);
        assert_eq!(tl.state, TrafficLightState::Red);
    }
}
