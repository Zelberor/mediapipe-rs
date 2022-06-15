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

type mFeatureType = mediagraph_FeatureType;
type mOutput = mediagraph_Output;
type mFeature = mediagraph_Feature;
type mFeatureList = mediagraph_FeatureList;

/// The type of visual feature made up of landmarks.
#[derive(Debug, Clone, Copy)]
pub enum FeatureType {
    Face,
    Hands,
    Pose,
}

impl FeatureType {
    fn num_landmarks(&self) -> usize {
        match self {
            FeatureType::Face => 478,
            FeatureType::Hands => 42,
            FeatureType::Pose => 33,
        }
    }
}

impl Into<mFeatureType> for FeatureType {
    fn into(self) -> mFeatureType {
        match self {
            FeatureType::Face => mediagraph_FeatureType_FACE,
            FeatureType::Hands => mediagraph_FeatureType_HANDS,
            FeatureType::Pose => mediagraph_FeatureType_POSE,
        }
    }
}

/// The definition of a graph output.
#[derive(Debug, Clone)]
pub struct Output {
    type_: FeatureType,
    name: String,
}

impl Into<mOutput> for Output {
    fn into(self) -> mOutput {
        let name = CString::new(self.name)
            .expect("CString::new failed")
            .into_raw();

        mOutput {
            type_: self.type_.into(),
            name,
        }
    }
}

/// The C++ mediagraph landmark type.
pub type Landmark = mediagraph_Landmark;

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
    outputs: Vec<Output>,
}

impl Detector {
    /// Creates a new Mediagraph with the given config.
    pub fn new(graph_config: &str, output_config: Vec<Output>) -> Self {
        assert!(
            output_config.len() > 0,
            "must specify at least one output feature"
        );
        let graph_config = CString::new(graph_config).expect("CString::new failed");

        let outputs = output_config
            .iter()
            .map(|f| f.clone().into())
            .collect::<Vec<mOutput>>();

        let graph: *mut mediagraph_Detector = unsafe {
            mediagraph_Detector::Create(
                graph_config.as_ptr(),
                outputs.as_ptr(),
                outputs.len() as u8,
            )
        };

        Self {
            graph,
            outputs: output_config,
        }
    }

    /// Processes the input frame, returns a slice of landmarks if any are detected.
    pub fn process(&mut self, input: &Mat) -> Vec<Vec<Vec<Landmark>>> {
        let mut data = input.clone();
        let results = unsafe {
            mediagraph_Detector_Process(
                self.graph as *mut std::ffi::c_void,
                data.data_mut(),
                data.cols(),
                data.rows(),
            )
        };

        let mut landmarks = vec![];

        let feature_lists =
            unsafe { std::slice::from_raw_parts(results, self.outputs.len() as usize) };

        for (i, feature_list) in feature_lists.iter().enumerate() {
            let num_landmarks = self.outputs[i].type_.num_landmarks();
            let mut fl = vec![];
            let features = unsafe {
                std::slice::from_raw_parts(
                    feature_list.features,
                    feature_list.num_features as usize,
                )
            };

            for feature in features.iter() {
                let landmarks = unsafe { std::slice::from_raw_parts(feature.data, num_landmarks) };
                fl.push(landmarks.to_vec());
            }

            landmarks.push(fl);
        }

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
