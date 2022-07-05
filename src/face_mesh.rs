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
                type_: FeatureType::Faces,
                name: "multi_face_landmarks".into(),
            }],
        );

        Self { graph }
    }

    /// Processes the input frame, returns a face mesh if detected.
    pub fn process(&mut self, input: &Mat) -> Vec<FaceMesh> {
        let landmarks = self.graph.process(input);
        let mut faces = vec![];

        for face_landmarks in landmarks[0].iter() {
            let mut face = FaceMesh::default();
            face.data.copy_from_slice(&face_landmarks[..]);
            faces.push(face);
        }

        faces
    }
}

impl Default for FaceMeshDetector {
    fn default() -> Self {
        Self::new()
    }
}
