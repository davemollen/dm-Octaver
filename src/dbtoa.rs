pub struct Dbtoa;

impl Dbtoa {
  pub fn process(input: f32) -> f32 {
    if input > -90.0 {
      10.0_f32.powf(input * 0.05)
    } else {
      0.0
    }
  }
}

#[cfg(test)]
mod dbtoa {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Dbtoa::process(-6.0), 0.5011872);
    assert_eq!(Dbtoa::process(-90.0), 0.0);
  }
}
