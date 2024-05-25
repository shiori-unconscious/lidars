use crate::{ImageBuffer, UnifiedTrait};
use anyhow::Result;

mod err_name;
use err_name::MV_ERR_NAME;

pub enum FlipFlag {
    None,
    Vertical,
    Horizontal,
    Both,
}

mod cam_op_ffi {
    extern "C" {
        pub fn initialize_camera(
            wanted_cam_number: u8,
            image_width: *mut u32,
            image_height: *mut u32,
            already_initialized: *mut u8,
            exposure_time: u32,
        ) -> u8;
        pub fn get_image(
            camera_index: u8,
            image_data: *mut u8,
            image_width: *mut u32,
            image_height: *mut u32,
            flip_flag: u8,
        ) -> u8;
        pub fn uninitialize_camera() -> u8;
    }
}

pub fn initialize_camera(
    wanted_cam_number: u8,
    buffer_width: &mut Vec<u32>,
    buffer_height: &mut Vec<u32>,
    exposure_time: u32,
) -> Result<()> {
    let mut already_initialized: u8 = 0;
    match unsafe {
        cam_op_ffi::initialize_camera(
            wanted_cam_number,
            buffer_width.as_mut_ptr(),
            buffer_height.as_mut_ptr(),
            &mut already_initialized as *mut u8,
            exposure_time,
        )
    } {
        0 => Ok(()),
        err_code => {
            if already_initialized != 0 {
                let _ = uninitialize_camera();
            }
            Err(anyhow::anyhow!(format!(
                "Failed to initialize camera, err code: {} ({})",
                err_code,
                MV_ERR_NAME
                    .get(err_code as usize)
                    .unwrap_or(&"err code unknown")
            )))
        }
    }
}

pub fn get_image(camera_index: u8, image: &mut ImageBuffer, flip_flag: FlipFlag) -> Result<()> {
    match unsafe {
        cam_op_ffi::get_image(
            camera_index,
            image.host(),
            &mut image.width as *mut u32,
            &mut image.height as *mut u32,
            flip_flag as u8,
        )
    } {
        0 => Ok(()),
        err_code => Err(anyhow::anyhow!(format!(
            "Failed to get image, err code: {} ({})",
            err_code,
            MV_ERR_NAME
                .get(err_code as usize)
                .unwrap_or(&"err code unknown")
        ))),
    }
}

pub fn uninitialize_camera() -> Result<()> {
    match unsafe { cam_op_ffi::uninitialize_camera() } {
        0 => Ok(()),
        err_code => Err(anyhow::anyhow!(format!(
            "Failed to uninitialize camera, err code: {} ({})",
            err_code,
            MV_ERR_NAME
                .get(err_code as usize)
                .unwrap_or(&"err code unknown")
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image() {
        let mut buffer_width = vec![0u32; 1];
        let mut buffer_height = vec![0u32; 1];

        if let Err(err) = initialize_camera(1, &mut buffer_width, &mut buffer_height, 1000) {
            panic!("initialize_camera failed err: {}", err);
        }

        let mut image = ImageBuffer::new(buffer_width[0], buffer_height[0]).unwrap();

        let vec: Vec<u8> = image.iter().map(|num| *num).collect();

        if let Err(err) = get_image(0, &mut image, FlipFlag::None) {
            println!("err: {}", err);
            panic!("get_image failed");
        }

        println!("height: {}, width: {}", image.height, image.width);

        assert!(!image.iter().zip(vec.iter()).all(|(a, b)| *a == *b));

        if let Err(err) = uninitialize_camera() {
            panic!("uninitialize_camera failed err: {}", err);
        }
    }
}
