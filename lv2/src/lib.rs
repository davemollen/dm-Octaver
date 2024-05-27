extern crate lv2;
extern crate octaver;
use lv2::prelude::*;
use octaver::{FloatExt, Octaver};

#[derive(PortCollection)]
struct Ports {
  threshold: InputPort<Control>,
  gain: InputPort<Control>,
  mix: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Octaver")]
struct DmOctaver {
  octaver: Octaver,
}

impl Plugin for DmOctaver {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      octaver: Octaver::new(_plugin_info.sample_rate() as f32),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let threshold = *ports.threshold.dbtoa();
    let gain = *ports.gain.dbtoa();
    let mix = *ports.mix * 0.01;

    for (in_frame, out_frame) in ports.input.iter().zip(ports.output.iter_mut()) {
      *out_frame = self.octaver.process(*in_frame, threshold, gain, mix);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmOctaver);
