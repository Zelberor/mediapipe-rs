//! Pose detection utilities.
use super::*;

/// Pose landmark indices.
pub enum PoseLandmark {
    NOSE = 0,
    LEFT_EYE_INNER = 1,
    LEFT_EYE = 2,
    LEFT_EYE_OUTER = 3,
    RIGHT_EYE_INNER = 4,
    RIGHT_EYE = 5,
    RIGHT_EYE_OUTER = 6,
    LEFT_EAR = 7,
    RIGHT_EAR = 8,
    MOUTH_LEFT = 9,
    MOUTH_RIGHT = 10,
    LEFT_SHOULDER = 11,
    RIGHT_SHOULDER = 12,
    LEFT_ELBOW = 13,
    RIGHT_ELBOW = 14,
    LEFT_WRIST = 15,
    RIGHT_WRIST = 16,
    LEFT_PINKY = 17,
    RIGHT_PINKY = 18,
    LEFT_INDEX = 19,
    RIGHT_INDEX = 20,
    LEFT_THUMB = 21,
    RIGHT_THUMB = 22,
    LEFT_HIP = 23,
    RIGHT_HIP = 24,
    LEFT_KNEE = 25,
    RIGHT_KNEE = 26,
    LEFT_ANKLE = 27,
    RIGHT_ANKLE = 28,
    LEFT_HEEL = 29,
    RIGHT_HEEL = 30,
    LEFT_FOOT_INDEX = 31,
    RIGHT_FOOT_INDEX = 32,
}

pub struct PoseDetector {
    graph: Detector,
}

impl PoseDetector {
    pub fn new() -> Self {
        let graph = Detector::new(
            include_str!("graphs/pose_tracking_cpu.pbtxt"),
            vec![Output {
                type_: FeatureType::Pose,
                name: "pose_landmarks".into(),
            }],
        );

        Self { graph }
    }

    /// Processes the input frame, returns a pose if detected.
    pub fn process(&mut self, input: &Mat) -> Option<Pose> {
        let result = self.graph.process(input);

        if result[0].is_empty() {
            return None;
        }

        let landmarks = &result[0][0];

        let mut pose = Pose::default();
        pose.data.copy_from_slice(landmarks.as_slice());
        Some(pose)
    }
}

impl Default for PoseDetector {
    fn default() -> Self {
        Self::new()
    }
}
