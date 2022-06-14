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
