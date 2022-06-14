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
pub mod face_mesh;
pub mod hands;
pub mod pose;
pub mod segmentation;

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
        let cols = data.cols();
        let rows = data.rows();
        let typ = data.typ();
        let out_data = unsafe {
            mediagraph_Effect_Process(
                self.graph as *mut std::ffi::c_void,
                data.data_mut(),
                cols,
                rows,
            )
        };
        unsafe {
            Mat::new_rows_cols_with_data(
                rows,
                cols,
                typ,
                out_data as *mut std::ffi::c_void,
                opencv::core::Mat_AUTO_STEP,
            )
            .unwrap()
        }
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        unsafe {
            mediagraph_Effect_Effect_destructor(self.graph);
        }
    }
}
