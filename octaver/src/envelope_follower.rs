mod slide;
use slide::Slide;

pub struct EnvelopeFollower {
  slide: Slide,
}

impl EnvelopeFollower {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      slide: Slide::new(sample_rate, 1., 60.),
    }
  }

  pub fn process(&mut self, input: f32, sidechain_input: f32) -> f32 {
    input * self.slide.process(sidechain_input.abs())
  }
}
