//! Hollistic detection utilities.
use super::*;

pub struct HolisticDetector {
    graph: Detector,
}

impl HolisticDetector {
    pub fn new() -> Self {
        let outputs = vec![
            Output {
                type_: FeatureType::Pose,
                name: "pose_landmarks",
            },
            Output {
                type_: FeatureType::Face,
                name: "face_landmarks",
            },
            Output {
                type_: FeatureType::Hand,
                name: "left_hand_landmarks",
            },
            Output {
                type_: FeatureType::Hand,
                name: "right_hand_landmarks",
            },
        ];

        let graph = Detector::new(include_str!("graphs/holistic_tracking_cpu.pbtxt"), outputs);

        Self { graph }
    }

    /// Processes the input frame, returns landmarks if detected
    pub fn process(&mut self, input: &Mat) -> Vec<Vec<Vec<Landmark>>> {
        let landmarks = self.graph.process(input);
        landmarks.clone()
    }
}

impl Default for HolisticDetector {
    fn default() -> Self {
        Self::new()
    }
}
