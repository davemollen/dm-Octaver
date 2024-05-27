use crate::FloatExt;

const ATTACK: f32 = 1.5;
const RELEASE: f32 = 30.;

pub struct Slide {
  slide_up: f32,
  slide_down: f32,
  z: f32,
}

impl Slide {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      slide_up: ATTACK.mstosamps(sample_rate).recip(),
      slide_down: RELEASE.mstosamps(sample_rate).recip(),
      z: 1.,
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    if input.is_equal_to(self.z) {
      input
    } else {
      let difference = input - self.z;
      self.z += difference
        * if input > self.z {
          self.slide_up
        } else {
          self.slide_down
        };
      self.z
    }
  }
}
