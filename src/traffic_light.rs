use crate::controller::Occupancy;

#[derive(PartialEq, Debug)]
pub enum TrafficLightState {
    Green,
    Red,
}

#[allow(async_fn_in_trait)]
pub trait TrafficLight {
    fn update(&mut self, occupancy: Occupancy);
}

pub struct SimpleTrafficLight {
    state: TrafficLightState,
}

impl SimpleTrafficLight {
    pub fn new() -> Self {
        SimpleTrafficLight {
            state: TrafficLightState::Green,
        }
    }
}

impl ToString for SimpleTrafficLight {
    fn to_string(&self) -> String {
        match self.state {
            TrafficLightState::Green => String::from("| |\n|G|"),
            TrafficLightState::Red => String::from("|R|\n| |"),
        }
    }
}

impl TrafficLight for SimpleTrafficLight {
    fn update(&mut self, status: Occupancy) {
        let Occupancy(occupied, total) = status;
        if occupied == total {
            self.state = TrafficLightState::Red;
        } else {
            self.state = TrafficLightState::Green;
        }
        println!("{}\n{}/{}", self.to_string(), occupied, total);
    }
}
