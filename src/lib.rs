#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(deref_nullptr)]

//! This library brings Google's Mediapipe to Rust.

// LINKING
#[link(name = "mediagraph")]
extern "C" {}

use opencv::prelude::*;
use std::ffi::CString;

mod bindings;

use bindings::*;

/// The C++ mediagraph graph type.
pub type DetectorType = mediagraph_DetectorType;

/// The C++ mediagraph landmark type.
pub type Landmark = mediagraph_Landmark;

pub const FACE_GRAPH_TYPE: DetectorType = mediagraph_DetectorType_FACE;
pub const HANDS_GRAPH_TYPE: DetectorType = mediagraph_DetectorType_HANDS;
pub const POSE_GRAPH_TYPE: DetectorType = mediagraph_DetectorType_POSE;

impl Default for Landmark {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            visibility: 0.0,
            presence: 0.0,
        }
    }
}

/// Represents a detected pose, as 33 landmarks.
/// Landmark names are in [pose::PoseLandmark].
pub struct Pose {
    pub data: [Landmark; 33],
}

impl Default for Pose {
    fn default() -> Self {
        Self {
            data: [Landmark::default(); 33],
        }
    }
}

/// Represents a detected hand, as 21 landmarks.
/// Landmark names are in [hands::HandLandmark]
#[derive(Default)]
pub struct Hand {
    pub data: [Landmark; 21],
}

/// Represents a detected face mesh, as 478 landmarks.
pub struct FaceMesh {
    pub data: [Landmark; 478],
}

impl Default for FaceMesh {
    fn default() -> Self {
        Self {
            data: [Landmark::default(); 478],
        }
    }
}

/// Detector calculator which interacts with the C++ library.
pub struct Detector {
    graph: *mut mediagraph_Detector,
    num_landmarks: u32,
}

impl Detector {
    /// Creates a new Mediagraph with the given config.
    pub fn new(graph_type: DetectorType, graph_config: &str, output_node: &str) -> Self {
        let graph_config = CString::new(graph_config).expect("CString::new failed");
        let output_node = CString::new(output_node).expect("CString::new failed");

        let graph: *mut mediagraph_Detector = unsafe {
            mediagraph_Detector::Create(graph_type, graph_config.as_ptr(), output_node.as_ptr())
        };

        let num_landmarks = match graph_type {
            FACE_GRAPH_TYPE => 478,
            HANDS_GRAPH_TYPE => 42,
            POSE_GRAPH_TYPE => 33,
            _ => 0,
        };

        Self {
            graph,
            num_landmarks,
        }
    }

    /// Processes the input frame, returns a slice of landmarks if any are detected.
    pub fn process(&mut self, input: &Mat) -> &[Landmark] {
        let mut data = input.clone();
        let raw_landmarks = unsafe {
            mediagraph_Detector_Process(
                self.graph as *mut std::ffi::c_void,
                data.data_mut(),
                data.cols(),
                data.rows(),
            )
        };
        if raw_landmarks.is_null() {
            return &[];
        }
        let landmarks =
            unsafe { std::slice::from_raw_parts(raw_landmarks, self.num_landmarks as usize) };
        landmarks
    }
}

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe {
            mediagraph_Detector_Detector_destructor(self.graph);
        }
    }
}

/// Effect calculator which interacts with the C++ library.
pub struct Effect {
    graph: *mut mediagraph_Effect,
}

impl Effect {
    /// Creates a new Mediagraph with the given config.
    pub fn new(graph_config: &str, output_node: &str) -> Self {
        let graph_config = CString::new(graph_config).expect("CString::new failed");
        let output_node = CString::new(output_node).expect("CString::new failed");

        let graph: *mut mediagraph_Effect =
            unsafe { mediagraph_Effect::Create(graph_config.as_ptr(), output_node.as_ptr()) };

        Self { graph }
    }

    /// Processes the input frame, returns a slice of landmarks if any are detected.
    pub fn process(&mut self, input: &Mat) -> Mat {
        let mut data = input.clone();
        let out_data = unsafe {
            mediagraph_Effect_Process(
                self.graph as *mut std::ffi::c_void,
                data.data_mut(),
                data.cols(),
                data.rows(),
            )
        };
        unsafe { Mat::from_raw(out_data as *mut std::ffi::c_void) }
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        unsafe {
            mediagraph_Effect_Effect_destructor(self.graph);
        }
    }
}

pub mod pose {
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
                POSE_GRAPH_TYPE,
                include_str!("graphs/pose_tracking_cpu.pbtxt"),
                "pose_landmarks",
            );

            Self { graph }
        }

        /// Processes the input frame, returns a pose if detected.
        pub fn process(&mut self, input: &Mat) -> Option<Pose> {
            let landmarks = self.graph.process(input);

            if landmarks.is_empty() {
                return None;
            }

            let mut pose = Pose::default();
            pose.data.copy_from_slice(landmarks);
            Some(pose)
        }
    }

    impl Default for PoseDetector {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod face_mesh {
    //! Face detection utilities.
    use super::*;

    pub struct FaceMeshDetector {
        graph: Detector,
    }

    impl FaceMeshDetector {
        pub fn new() -> Self {
            let graph = Detector::new(
                FACE_GRAPH_TYPE,
                include_str!("graphs/face_mesh_desktop_live.pbtxt"),
                "multi_face_landmarks",
            );

            Self { graph }
        }

        /// Processes the input frame, returns a face mesh if detected.
        pub fn process(&mut self, input: &Mat) -> Option<FaceMesh> {
            let landmarks = self.graph.process(input);

            if landmarks.is_empty() {
                return None;
            }

            let mut face_mesh = FaceMesh::default();
            face_mesh.data.copy_from_slice(landmarks);
            Some(face_mesh)
        }
    }

    impl Default for FaceMeshDetector {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod hands {
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
                HANDS_GRAPH_TYPE,
                include_str!("graphs/hand_tracking_desktop_live.pbtxt"),
                "hand_landmarks",
            );

            Self { graph }
        }

        /// Processes the input frame, returns a tuple of hands if detected.
        pub fn process(&mut self, input: &Mat) -> Option<[Hand; 2]> {
            let landmarks = self.graph.process(input);

            if landmarks.is_empty() {
                return None;
            }

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
}

pub mod segmentation {
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
}
