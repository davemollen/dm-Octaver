use crate::shared::slide::Slide;

pub struct NoiseGate {
  slide: Slide,
}

impl NoiseGate {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      slide: Slide::new(sample_rate, 1.5, 30.),
    }
  }

  pub fn process(&mut self, input: f32, threshold: f32) -> f32 {
    let is_above_threshold = if input.abs() > threshold { 1. } else { 0. };
    let envelope = self.slide.process(is_above_threshold);

    input * envelope
  }
}
