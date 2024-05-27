pub fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

pub fn generate_signal_stream(length: usize) -> Vec<f32> {
  (0..length).map(|_| generate_signal()).collect()
}
