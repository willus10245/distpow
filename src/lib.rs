#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct DistPow {
    params: Arc<DistPowParams>,
}

struct DistPowParams {
    threshold: AtomicFloat,
}

impl Default for DistPow {
    fn default() -> DistPow {
        DistPow {
            params: Arc::new(DistPowParams::default()),
        }
    }
}

impl Default for DistPowParams {
    fn default() -> DistPowParams {
        DistPowParams {
            threshold: AtomicFloat::new(1.0),
        }
    }
}

impl Plugin for DistPow {
    fn get_info(&self) -> Info {
        Info {
            name: "DistPow".to_string(),
            vendor: "dis guy".to_string(),
            unique_id: 12022019,

            inputs: 2,
            outputs: 2,
            parameters: 1,

            ..Info::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let threshold = self.params.threshold.get();

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(threshold) / threshold;
                } else {
                    *output_sample = input_sample.max(-threshold) / threshold;
                }
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for DistPowParams {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.threshold.set(value.max(0.01)),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold",
            _ => "",
        }
        .to_string()
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.threshold.get() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%",
            _ => "",
        }
        .to_string()
    }
}

plugin_main!(DistPow);
