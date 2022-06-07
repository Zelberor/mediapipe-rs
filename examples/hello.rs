#![allow(unused_variables)]
#![allow(dead_code)]

mod examples {
    use super::*;
    use opencv::prelude::*;
    use opencv::{highgui, imgproc, videoio, Result};

    pub fn corner_rectangle() -> Result<()> {
        let window = "video capture";

        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

        let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            panic!("Unable to open default cam")
        }

        let detector = hands::HandDetector::default();

        loop {
            let mut frame = Mat::default();
            cap.read(&mut frame)?;
            let size = frame.size()?;
            if size.width > 0 {
                highgui::imshow(window, &mut frame)?
            }
            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
        Ok(())
    }

    pub fn face_mesh() -> Result<()> {
        let window = "video capture";

        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

        let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            panic!("Unable to open default cam")
        }

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
        cap.set(videoio::CAP_PROP_FPS, 30.0)?;

        let mut mesh = FaceMesh::default();
        let mut detector = face_mesh::FaceMeshDetector::default();

        let mut raw_frame = Mat::default();
        let mut rgb_frame = Mat::default();
        let mut flip_frame = Mat::default();
        loop {
            cap.read(&mut raw_frame)?;

            let size = raw_frame.size()?;
            if size.width > 0 && !raw_frame.empty() {
                imgproc::cvt_color(&raw_frame, &mut rgb_frame, imgproc::COLOR_BGR2RGB, 0)?;
                opencv::core::flip(&rgb_frame, &mut flip_frame, 1)?; // horizontal

                println!("processing");
                detector.process(&flip_frame, &mut mesh);

                highgui::imshow(window, &mut flip_frame)?;
                println!(
                    "LANDMARK: {} {} {}",
                    mesh.data[0].x, mesh.data[0].y, mesh.data[0].z
                );
            } else {
                println!("WARN: Skip empty frame");
            }

            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
        Ok(())
    }

    pub fn hand_tracking() -> Result<()> {
        let window = "video capture";

        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

        let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            panic!("Unable to open default cam")
        }

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
        cap.set(videoio::CAP_PROP_FPS, 30.0)?;

        let mut left = Hand::default();
        let mut right = Hand::default();
        let mut detector = hands::HandDetector::default();

        let mut raw_frame = Mat::default();
        let mut rgb_frame = Mat::default();
        let mut flip_frame = Mat::default();
        loop {
            cap.read(&mut raw_frame)?;

            let size = raw_frame.size()?;
            if size.width > 0 && !raw_frame.empty() {
                imgproc::cvt_color(&raw_frame, &mut rgb_frame, imgproc::COLOR_BGR2RGB, 0)?;
                opencv::core::flip(&rgb_frame, &mut flip_frame, 1)?; // horizontal

                detector.process(&flip_frame, &mut left, &mut right);

                highgui::imshow(window, &mut flip_frame)?;
                println!(
                    "LANDMARK: {} {} {}",
                    left.data[0].x, left.data[0].y, left.data[0].z
                );
            } else {
                println!("WARN: Skip empty frame");
            }

            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
        Ok(())
    }

    pub fn pose_estimation() -> Result<()> {
        let window = "video capture";

        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

        let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            panic!("Unable to open default cam")
        }

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
        cap.set(videoio::CAP_PROP_FPS, 30.0)?;

        let mut pose = Pose::default();
        let mut detector = pose::PoseDetector::default();

        let mut raw_frame = Mat::default();
        let mut rgb_frame = Mat::default();
        let mut flip_frame = Mat::default();
        loop {
            cap.read(&mut raw_frame)?;

            let size = raw_frame.size()?;
            if size.width > 0 && !raw_frame.empty() {
                imgproc::cvt_color(&raw_frame, &mut rgb_frame, imgproc::COLOR_BGR2RGB, 0)?;
                opencv::core::flip(&rgb_frame, &mut flip_frame, 1)?; // horizontal

                detector.process(&rgb_frame, &mut pose);

                highgui::imshow(window, &mut rgb_frame)?;
                println!(
                    "LANDMARK: {} {} {}",
                    pose.data[0].x, pose.data[0].y, pose.data[0].z
                );
            } else {
                println!("WARN: Skip empty frame");
            }

            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
        Ok(())
    }
}

fn main() {
    // examples::pose_estimation().unwrap()
    // examples::hand_tracking().unwrap()
    examples::face_mesh().unwrap()
}
