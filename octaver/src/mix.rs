use crate::shared::float_ext::FloatExt;
use std::f32::consts::FRAC_PI_2;

pub struct Mix;

impl Mix {
  pub fn process(dry: f32, wet: f32, mix: f32) -> f32 {
    let factor = mix * FRAC_PI_2;
    let dry_gain = factor.fast_cos();
    let wet_gain = factor.fast_sin();
    dry * dry_gain + wet * wet_gain
  }
}
