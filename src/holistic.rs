//! Hollistic detection utilities.
use super::*;

pub struct HolisticDetector {
    graph: Detector,
}

#[derive(Clone, Debug)]
pub struct HolisticDetection {
    pub pose: Option<Pose>,
    pub face: Option<FaceMesh>,
    pub left_hand: Option<Hand>,
    pub right_hand: Option<Hand>,
}

impl HolisticDetector {
    pub fn new() -> Self {
        let outputs = vec![
            Output {
                type_: FeatureType::Pose,
                name: "pose_landmarks".into(),
            },
            Output {
                type_: FeatureType::Face,
                name: "face_landmarks".into(),
            },
            Output {
                type_: FeatureType::Hand,
                name: "left_hand_landmarks".into(),
            },
            Output {
                type_: FeatureType::Hand,
                name: "right_hand_landmarks".into(),
            },
        ];

        let graph = Detector::new(include_str!("graphs/holistic_tracking_cpu.pbtxt"), outputs);

        Self { graph }
    }

    /// Processes the input frame, returns landmarks if detected
    pub fn process(&mut self, input: &Mat) -> HolisticDetection {
        let landmarks = self.graph.process(input);

        let mut pose = None;
        let mut face = None;
        let mut left_hand = None;
        let mut right_hand = None;

        if !landmarks[0].is_empty() {
            let mut p = Pose::default();
            p.data.copy_from_slice(&landmarks[0][0][..]);
            pose = Some(p);
        }

        if !landmarks[1].is_empty() {
            let mut f = FaceMesh::default();
            f.data.copy_from_slice(&landmarks[1][0][..]);
            face = Some(f);
        }

        if !landmarks[2].is_empty() {
            let mut l = Hand::default();
            l.data.copy_from_slice(&landmarks[2][0][..]);
            left_hand = Some(l);
        }

        if !landmarks[3].is_empty() {
            let mut r = Hand::default();
            r.data.copy_from_slice(&landmarks[3][0][..]);
            right_hand = Some(r);
        }

        HolisticDetection {
            pose,
            face,
            left_hand,
            right_hand,
        }
    }
}

impl Default for HolisticDetector {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MultiPersonHolisticDetector {
    graph: Detector,
}

impl MultiPersonHolisticDetector {
    pub fn new() -> Self {
        let outputs = vec![
            Output {
                type_: FeatureType::Poses,
                name: "multi_pose_landmarks".into(),
            },
            Output {
                type_: FeatureType::Faces,
                name: "multi_face_landmarks".into(),
            },
            Output {
                type_: FeatureType::Hands,
                name: "multi_left_hand_landmarks".into(),
            },
            Output {
                type_: FeatureType::Hands,
                name: "multi_right_hand_landmarks".into(),
            },
        ];

        let graph = Detector::new(
            include_str!("graphs/multi_person_holistic_tracking_cpu.pbtxt"),
            outputs,
        );

        Self { graph }
    }

    /// Processes the input frame, returns landmarks if detected
    pub fn process(&mut self, input: &Mat) -> Vec<HolisticDetection> {
        let landmarks = self.graph.process(input);

        let max_landmarks = landmarks
            .iter()
            .map(|l| l.len())
            .reduce(|acc, item| acc.max(item))
            .unwrap();

        let mut detections = vec![];

        for i in 0..max_landmarks {
            let mut pose = None;
            let mut face = None;
            let mut left_hand = None;
            let mut right_hand = None;

            if landmarks[0].len() > i {
                let mut p = Pose::default();
                p.data.copy_from_slice(&landmarks[0][i][..]);
                pose = Some(p);
            }

            if landmarks[1].len() > i {
                let mut f = FaceMesh::default();
                f.data.copy_from_slice(&landmarks[1][i][..]);
                face = Some(f);
            }

            if landmarks[2].len() > i {
                let mut l = Hand::default();
                l.data.copy_from_slice(&landmarks[2][i][..]);
                left_hand = Some(l);
            }

            if landmarks[3].len() > i {
                let mut r = Hand::default();
                r.data.copy_from_slice(&landmarks[3][i][..]);
                right_hand = Some(r);
            }

            detections.push(HolisticDetection {
                pose,
                face,
                left_hand,
                right_hand,
            });
        }

        detections
    }
}

impl Default for MultiPersonHolisticDetector {
    fn default() -> Self {
        Self::new()
    }
}
