//! Selfie segmentation utilities.
use super::*;

pub struct Segmentor {
    graph: Effect,
}

impl Segmentor {
    pub fn new() -> Self {
        let graph = Effect::new(
            include_str!("graphs/selfie_segmentation_cpu.pbtxt"),
            "output_video",
        );

        Self { graph }
    }

    /// Processes the input frame, returns the output frame.
    pub fn process(&mut self, input: &Mat) -> Mat {
        self.graph.process(input)
    }
}

impl Default for Segmentor {
    fn default() -> Self {
        Self::new()
    }
}
