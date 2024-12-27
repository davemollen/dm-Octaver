mod smooth;
pub use smooth::Smoother;
use {
  crate::shared::float_ext::FloatExt,
  smooth::{ExponentialSmooth, LinearSmooth},
};

pub struct Params {
  pub gain: ExponentialSmooth,
  pub mix: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      gain: ExponentialSmooth::new(sample_rate, 12.),
      mix: LinearSmooth::new(sample_rate, 12.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, gain: f32, mix: f32) {
    let gain = gain.dbtoa();

    if self.is_initialized {
      self.gain.set_target(gain);
      self.mix.set_target(mix);
    } else {
      self.gain.reset(gain);
      self.mix.reset(mix);
      self.is_initialized = true;
    }
  }
}
