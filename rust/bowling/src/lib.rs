#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Eq, PartialEq, Debug)]
enum Frame {
    Open(u16, u16),
    Last(u16),
    Spare(u16, u16),
    Strike,
}

#[derive(Default)]
pub struct BowlingGame {
    roll: Option<u16>,
    frames: Vec<Frame>,
    finished: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if let Some(n) = self.roll {
            if n + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        if let Some(Frame::Last(n)) = self.frames.last() {
            if n + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        if self.finished {
            return Err(Error::GameComplete);
        }

        let (frame, roll) = {
            if let Some(n) = self.roll {
                if n + pins == 10 {
                    (Some(Frame::Spare(n, pins)), None)
                } else {
                    (Some(Frame::Open(n, pins)), None)
                }
            } else if pins == 10 {
                (Some(Frame::Strike), None)
            } else if self.frames.len() >= 10 {
                (Some(Frame::Last(pins)), None)
            } else {
                (None, Some(pins))
            }
        };

        self.finished = match (self.frames.len(), self.frames.last(), &frame) {
            (11, _, _) => true,
            (10, Some(Frame::Spare(_, _)), _) => true,
            (_, _, Some(Frame::Strike)) => false,
            (9, _, Some(Frame::Spare(_, _))) => false,
            (9, _, None) => false,
            (9, _, _) => true,
            _ => false,
        };
        if let Some(frame) = frame {
            self.frames.push(frame);
        }
        self.roll = roll;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.finished {
            return None;
        }

        let mut score = 0;
        for i in 0..9 {
            score += match (
                self.frames.get(i),
                self.frames.get(i + 1),
                self.frames.get(i + 2),
            ) {
                (Some(Frame::Open(a, b)), _, _) => a + b,
                (Some(Frame::Spare(_, _)), Some(Frame::Open(a, _)), _) => 10 + a,
                (Some(Frame::Spare(_, _)), Some(Frame::Spare(a, _)), _) => 10 + a,
                (Some(Frame::Spare(_, _)), Some(Frame::Strike), Some(Frame::Open(a, _))) => 20 + a,
                (Some(Frame::Spare(_, _)), Some(Frame::Strike), Some(Frame::Spare(a, _))) => 20 + a,
                (Some(Frame::Strike), Some(Frame::Open(a, b)), _) => 10 + a + b,
                (Some(Frame::Strike), Some(Frame::Spare(_, _)), _) => 20,
                (Some(Frame::Strike), Some(Frame::Strike), Some(Frame::Open(a, _))) => 20 + a,
                (Some(Frame::Strike), Some(Frame::Strike), Some(Frame::Spare(a, _))) => 20 + a,
                (Some(Frame::Strike), Some(Frame::Strike), Some(Frame::Strike)) => 30,
                (Some(Frame::Last(a)), _, _) => *a,
                _ => 0,
            };
        }

        if let Some(last_frames) = self.frames.get(9..) {
            for last_frame in last_frames {
                score += match last_frame {
                    Frame::Open(a, b) => a + b,
                    Frame::Spare(a, b) => a + b,
                    Frame::Strike => 10,
                    Frame::Last(a) => *a,
                };
            }
        };

        Some(score)
    }
}
