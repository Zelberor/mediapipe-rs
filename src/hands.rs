//! Hand detection utilities.
use super::*;

/// Hand landmark indices.
pub enum HandLandmark {
    WRIST = 0,
    THUMB_CMC = 1,
    THUMB_MCP = 2,
    THUMB_IP = 3,
    THUMB_TIP = 4,
    INDEX_FINGER_MCP = 5,
    INDEX_FINGER_PIP = 6,
    INDEX_FINGER_DIP = 7,
    INDEX_FINGER_TIP = 8,
    MIDDLE_FINGER_MCP = 9,
    MIDDLE_FINGER_PIP = 10,
    MIDDLE_FINGER_DIP = 11,
    MIDDLE_FINGER_TIP = 12,
    RING_FINGER_MCP = 13,
    RING_FINGER_PIP = 14,
    RING_FINGER_DIP = 15,
    RING_FINGER_TIP = 16,
    PINKY_MCP = 17,
    PINKY_PIP = 18,
    PINKY_DIP = 19,
    PINKY_TIP = 20,
}

pub struct HandDetector {
    graph: Detector,
}

impl HandDetector {
    pub fn new() -> Self {
        let graph = Detector::new(
            include_str!("graphs/hand_tracking_desktop_live.pbtxt"),
            vec![Output {
                type_: FeatureType::Hands,
                name: "hand_landmarks".into(),
            }],
        );

        Self { graph }
    }

    /// Processes the input frame, returns a tuple of hands if detected.
    pub fn process(&mut self, input: &Mat) -> Option<[Hand; 2]> {
        let result = self.graph.process(input);

        if result[0].is_empty() {
            return None;
        }

        let landmarks = &result[0][0];

        let mut lh = Hand::default();
        let mut rh = Hand::default();
        lh.data.copy_from_slice(&landmarks[0..21]);
        rh.data.copy_from_slice(&landmarks[21..42]);

        Some([lh, rh])
    }
}

impl Default for HandDetector {
    fn default() -> Self {
        Self::new()
    }
}
