use crate::controller::Occupancy;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

#[derive(PartialEq, Debug)]
pub enum TrafficLightState {
    Green,
    Yellow,
    Red,
}

pub trait TrafficLight {
    fn update(&mut self, occupancy: Occupancy);
}

pub struct TerminalTrafficLight {
    state: TrafficLightState,
}

impl TerminalTrafficLight {
    pub fn new() -> Self {
        TerminalTrafficLight {
            state: TrafficLightState::Green,
        }
    }

    fn display(&self) {
        let mut stdout = stdout();
        // Move cursor to the top-left corner
        execute!(stdout, MoveTo(0, 0)).unwrap();
        // Clear the area where the traffic light will be displayed
        execute!(stdout, Clear(ClearType::FromCursorDown)).unwrap();
        match self.state {
            TrafficLightState::Green => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print("| |\n| |\n|G|"),
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();
            }
            TrafficLightState::Yellow => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print("| |\n|Y|\n| |"),
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();
            }
            TrafficLightState::Red => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Red),
                    Print("|R|\n| |\n| |"),
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}

impl TrafficLight for TerminalTrafficLight {
    fn update(&mut self, status: Occupancy) {
        let Occupancy(occupied, total) = status;
        if occupied == total {
            self.state = TrafficLightState::Red;
        } else if occupied > 0 {
            self.state = TrafficLightState::Yellow;
        } else {
            self.state = TrafficLightState::Green
        }
        self.display();
        println!("\n{}/{}", occupied, total);
    }
}
