//! Face detection utilities.
use super::*;

pub struct FaceMeshDetector {
    graph: Detector,
}

impl FaceMeshDetector {
    pub fn new() -> Self {
        let graph = Detector::new(
            include_str!("graphs/face_mesh_desktop_live.pbtxt"),
            vec![Output {
                type_: FeatureType::Face,
                name: "multi_face_landmarks".into(),
            }],
        );

        Self { graph }
    }

    /// Processes the input frame, returns a face mesh if detected.
    pub fn process(&mut self, input: &Mat) -> Vec<Vec<Landmark>> {
        let landmarks = self.graph.process(input);
        landmarks[0].clone()
    }
}

impl Default for FaceMeshDetector {
    fn default() -> Self {
        Self::new()
    }
}
