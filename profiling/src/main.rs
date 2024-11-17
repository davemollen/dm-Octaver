mod utils;
use octaver::Octaver;
use utils::generate_signal;

fn main() {
  let mut octaver = Octaver::new(44100.);

  loop {
    let input = generate_signal();
    octaver.process(input, 1., 0.75);
  }
}
