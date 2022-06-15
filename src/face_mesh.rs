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
    pub fn process(&mut self, input: &Mat) -> Option<FaceMesh> {
        let landmarks = self.graph.process(input);

        if landmarks[0].is_empty() {
            return None;
        }

        let mut face_mesh = FaceMesh::default();
        face_mesh.data.copy_from_slice(landmarks[0][0].as_slice());
        Some(face_mesh)
    }
}

impl Default for FaceMeshDetector {
    fn default() -> Self {
        Self::new()
    }
}
