#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    rolls: Vec<u16>,
    current_frame: Frame,
}
#[derive(Clone)]
pub struct Frame {
    rolls: Vec<u16>,
}

impl Frame {
    fn pins_left(&self) -> u16 {
        self.rolls.clone().into_iter().sum()
    }
    fn add_roll(mut self, roll: u16) {
        self.rolls.push(roll);
    }
    fn rolls_made(&self) -> u16 {
        self.rolls.len() as u16
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![],
            rolls: vec![],
            current_frame: Frame { rolls: vec![] },
        }
    }

    pub fn roll(&self, pins: u16) -> Result<(), Error> {
        // self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        todo!("Return the score if the game is complete, or None if not.");
    }
}
