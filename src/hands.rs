//! Hand detection utilities.
use super::*;

pub const NUM_HAND_LANDMARKS: usize = 21;

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

    /// Processes the input frame, returns a list of hands
    pub fn process(&mut self, input: &mut Mat) -> Vec<Hand> {
        let result = self.graph.process(input);
        let mut hands = vec![];

        for hand_landmarks in result[0].iter() {
            let mut hand = Hand::default();
            hand.data.copy_from_slice(&hand_landmarks[..]);
            hands.push(hand);
        }

        hands
    }
}

impl Default for HandDetector {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HandVisualizer {
    graph: Effect,
}

impl HandVisualizer {
    pub fn new() -> Self {
        let graph = Effect::new(include_str!("graphs/hand_tracking_desktop_live.pbtxt"), "output_video");

        Self { graph }
    }

    /// Processes the input frame, returns the output frame.
    pub fn process(&mut self, input: &mut Mat) -> Mat {
        self.graph.process(input)
    }
}
