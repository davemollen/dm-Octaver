extern crate lv2;
extern crate octaver;
use lv2::prelude::*;
use octaver::{Octaver, Params};

#[derive(PortCollection)]
struct Ports {
  gain: InputPort<Control>,
  mix: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Octaver")]
struct DmOctaver {
  octaver: Octaver,
  params: Params,
}

impl Plugin for DmOctaver {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;
    Some(Self {
      octaver: Octaver::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(*ports.gain, *ports.mix * 0.01);

    for (in_frame, out_frame) in ports.input.iter().zip(ports.output.iter_mut()) {
      *out_frame = self.octaver.process(*in_frame, &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmOctaver);
