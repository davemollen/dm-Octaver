use super::clip::Clip;
use super::delta::Delta;
use super::lowpass::Lowpass;
use super::mix::Mix;
use super::noise_gate::NoiseGate;

pub struct Octaver {
  lowpass: Lowpass,
  delta: Delta,
  noise_gate: NoiseGate,
  flip_flop: f32,
}

impl Octaver {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: Lowpass::new(sample_rate),
      delta: Delta::new(),
      noise_gate: NoiseGate::new(sample_rate),
      flip_flop: 1.,
    }
  }
  pub fn process(&mut self, input: f32, threshold: f32, gain: f32, mix: f32) -> f32 {
    let gate = self.noise_gate.processinput, threshold, 1.5, 30.);
    let lowpass = self.lowpass.processgate, 2.);
    let clip = Clip::process(lowpass * 10000., -1., 1.);
    let is_below_zero = if clip < 0. { 1. } else { 0. };
    let trigger = self.delta.processis_below_zero) > 0.;
    if trigger {
      if self.flip_flop == 1. {
        self.flip_flop = -1.
      } else {
        self.flip_flop = 1.
      }
    };
    let octaver = clip * self.flip_flop * gain;
    Mix::process(input, octaver, mix)
  }
}
