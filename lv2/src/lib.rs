extern crate lv2;
extern crate octaver;
use lv2::prelude::*;
use octaver::{Octaver, Params};

#[derive(PortCollection)]
struct Ports {
  gain: InputPort<InPlaceControl>,
  mix: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
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
    self.params.set(ports.gain.get(), ports.mix.get() * 0.01);

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let octaver_output = self.octaver.process(input.get(), &mut self.params);
      output.set(octaver_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmOctaver);
