mod delta;
mod mix;
mod noise_gate;
mod one_pole_filter;
mod shared {
  pub mod float_ext;
  pub mod slide;
}
mod envelope_follower;
pub use shared::float_ext::FloatExt;
use {
  delta::Delta, envelope_follower::EnvelopeFollower, mix::Mix, noise_gate::NoiseGate,
  one_pole_filter::OnePoleFilter,
};

pub struct Octaver {
  lowpass_filter: OnePoleFilter,
  delta: Delta,
  noise_gate: NoiseGate,
  flip_flop: f32,
  envelope_follower: EnvelopeFollower,
}

impl Octaver {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass_filter: OnePoleFilter::new(sample_rate, 2.),
      delta: Delta::new(),
      noise_gate: NoiseGate::new(sample_rate),
      flip_flop: 1.,
      envelope_follower: EnvelopeFollower::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, threshold: f32, gain: f32, mix: f32) -> f32 {
    let gate_output = self.noise_gate.process(input, threshold);
    let lowpass_filter_output = self.lowpass_filter.process(gate_output);
    let clip_output = (lowpass_filter_output * 10000.).clamp(-1., 1.);
    let trigger = self.delta.process(clip_output.signum()) > 0.;
    if trigger {
      if self.flip_flop == 1. {
        self.flip_flop = -1.
      } else {
        self.flip_flop = 1.
      }
    };
    let octaver = clip_output * self.flip_flop * gain;
    let wet = self.envelope_follower.process(octaver, input);

    Mix::process(input, wet, mix)
  }
}
