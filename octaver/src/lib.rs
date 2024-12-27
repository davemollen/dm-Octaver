mod delta;
mod mix;
mod one_pole_filter;
mod params;
mod shared {
  pub mod float_ext;
}
mod envelope_follower;
pub use params::Params;
use {
  delta::Delta, envelope_follower::EnvelopeFollower, mix::Mix, one_pole_filter::OnePoleFilter,
  params::Smoother, shared::float_ext::FloatExt,
};

pub struct Octaver {
  lowpass_filter: OnePoleFilter,
  delta: Delta,
  flip_flop: f32,
  envelope_follower: EnvelopeFollower,
}

impl Octaver {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass_filter: OnePoleFilter::new(sample_rate, 2.),
      delta: Delta::new(),
      flip_flop: 1.,
      envelope_follower: EnvelopeFollower::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let gain = params.gain.next();
    let mix = params.mix.next();

    let lowpass_filter_output = self.lowpass_filter.process(input);
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
