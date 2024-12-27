mod utils;
use octaver::{Octaver, Params};
use utils::generate_signal;

fn main() {
  let mut octaver: Octaver = Octaver::new(44100.);
  let mut params = Params::new(44100.);
  params.set(1., 0.75);

  loop {
    let input = generate_signal();
    octaver.process(input, &mut params);
  }
}
