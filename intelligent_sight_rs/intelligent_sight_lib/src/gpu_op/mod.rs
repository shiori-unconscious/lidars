mod err_code;

use crate::{ImageBuffer, TensorBuffer, UnifiedItem, UnifiedTrait};
use anyhow::{anyhow, Result};
use err_code::{CUDA_ERR_NAME, TRT_ERR_NAME};
use std::mem;

mod cuda_op_ffi {
    extern "C" {
        #[cfg(any(target_os = "windows", target_arch = "aarch64"))]
        pub fn cuda_malloc(size: u32, ptr: *mut *mut u8) -> u16;
        #[cfg(not(any(target_os = "windows", target_arch = "aarch64")))]
        pub fn cuda_malloc_managed(size: u32, ptr: *mut *mut u8) -> u16;
        #[cfg(any(target_os = "windows", target_arch = "aarch64"))]
        pub fn cuda_malloc_host(size: u32, ptr: *mut *mut u8) -> u16;
        pub fn cuda_free(ptr: *mut u8) -> u16;
        #[cfg(any(target_os = "windows", target_arch = "aarch64"))]
        pub fn cuda_free_host(ptr: *mut u8) -> u16;
        pub fn transfer_host_to_device(
            host_buffer: *const u8,
            device_buffer: *mut u8,
            size: u32,
        ) -> u16;
        pub fn transfer_device_to_host(
            host_buffer: *mut u8,
            device_buffer: *const u8,
            size: u32,
        ) -> u16;
        pub fn convert_rgb888_3dtensor(
            input_buffer: *const u8,
            output_buffer: *mut f32,
            width: u32,
            height: u32,
        ) -> u16;
    }
}

mod trt_op_ffi {
    use std::ffi::c_char;
    extern "C" {
        pub fn create_engine(
            engine_filename: *const c_char,
            input_name: *const c_char,
            output_name: *const c_char,
            width: u32,
            height: u32,
        ) -> u16;
        pub fn create_context() -> u16;
        pub fn infer() -> u16;
        pub fn release_resources() -> u16;
        pub fn set_input(input_buffer: *mut f32) -> u16;
        pub fn set_output(output_buffer: *mut f32) -> u16;
        pub fn postprocess_init(
            max_detect: u16,
            conf_threshold: f32,
            iou_threshold: f32,
            feature_map_size: u16,
        ) -> u16;
        pub fn postprocess_init_default() -> u16;
        pub fn postprocess(
            input_buffer: *const f32,
            output_buffer: *mut f32,
            num_detections: *mut u16,
        ) -> u16;
        pub fn postprocess_destroy() -> u16;
    }
}

#[inline]
fn exec_and_check(mut f: impl FnMut() -> Result<u16>) -> Result<()> {
    match f()? {
        0 => Ok(()),
        err @ 1..=9999 => Err(anyhow!(
            "GPU: Failed, cuda error code: {} ({})",
            err,
            CUDA_ERR_NAME
                .get(err as usize)
                .unwrap_or(&"err code unknown")
        )),
        err => Err(anyhow!(
            "GPU: Failed, customized error code: {} ({})",
            err,
            TRT_ERR_NAME
                .get(err as usize)
                .unwrap_or(&"err code unknown")
        )),
    }
}

#[cfg(any(target_os = "windows", target_arch = "aarch64"))]
pub fn cuda_malloc<T>(size: usize) -> Result<*mut T>
where
    T: Sized,
{
    let mut ptr = std::ptr::null_mut();
    exec_and_check(|| {
        Ok(unsafe {
            cuda_op_ffi::cuda_malloc(
                (size * mem::size_of::<T>() / mem::size_of::<u8>()) as u32,
                &mut ptr as *mut *mut T as *mut *mut u8,
            )
        })
    })
    .map(|_| ptr)
}

#[cfg(any(target_os = "windows", target_arch = "aarch64"))]
pub fn cuda_malloc_host<T>(size: usize) -> Result<*mut T>
where
    T: Sized,
{
    let mut ptr = std::ptr::null_mut();
    exec_and_check(|| {
        Ok(unsafe {
            cuda_op_ffi::cuda_malloc_host(
                (size * mem::size_of::<T>() / mem::size_of::<u8>()) as u32,
                &mut ptr as *mut *mut T as *mut *mut u8,
            )
        })
    })
    .map(|_| ptr)
}

#[cfg(not(any(target_os = "windows", target_arch = "aarch64")))]
pub fn cuda_malloc_managed<T>(size: usize) -> Result<*mut T>
where
    T: Sized,
{
    let mut ptr = std::ptr::null_mut();
    exec_and_check(|| {
        Ok(unsafe {
            cuda_op_ffi::cuda_malloc_managed(
                (size * mem::size_of::<T>() / mem::size_of::<u8>()) as u32,
                &mut ptr as *mut *mut T as *mut *mut u8,
            )
        })
    })
    .map(|_| ptr)
}

pub fn cuda_free<T>(ptr: *mut T) -> Result<()> {
    exec_and_check(|| Ok(unsafe { cuda_op_ffi::cuda_free(ptr as *mut u8) }))
}

#[cfg(any(target_os = "windows", target_arch = "aarch64"))]
pub fn cuda_free_host<T>(ptr: *mut T) -> Result<()> {
    exec_and_check(|| Ok(unsafe { cuda_op_ffi::cuda_free_host(ptr as *mut u8) }))
}

pub fn convert_rgb888_3dtensor(
    input_image: &mut ImageBuffer,
    output_tensor: &mut TensorBuffer,
) -> Result<()> {
    exec_and_check(|| {
        Ok(unsafe {
            cuda_op_ffi::convert_rgb888_3dtensor(
                input_image.to_device()?,
                output_tensor.device()?,
                input_image.width,
                input_image.height,
            )
        })
    })
}

pub fn transfer_host_to_device<T>(
    host_buffer: *const T,
    device_buffer: *mut T,
    size: usize,
) -> Result<()> {
    exec_and_check(|| {
        Ok(unsafe {
            cuda_op_ffi::transfer_host_to_device(
                host_buffer as *const u8,
                device_buffer as *mut u8,
                (size * mem::size_of::<T>() / mem::size_of::<u8>()) as u32,
            )
        })
    })
}

pub fn transfer_device_to_host<T>(
    host_buffer: *mut T,
    device_buffer: *const T,
    size: usize,
) -> Result<()> {
    exec_and_check(|| {
        Ok(unsafe {
            cuda_op_ffi::transfer_device_to_host(
                host_buffer as *mut u8,
                device_buffer as *const u8,
                (size * mem::size_of::<T>() / mem::size_of::<u8>()) as u32,
            )
        })
    })
}

pub fn create_engine(
    engine_filename: &str,
    input_name: &str,
    output_name: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let engine_filename = std::ffi::CString::new(engine_filename)?;
    let input_name = std::ffi::CString::new(input_name)?;
    let output_name = std::ffi::CString::new(output_name)?;
    exec_and_check(|| {
        Ok(unsafe {
            trt_op_ffi::create_engine(
                engine_filename.as_ptr(),
                input_name.as_ptr(),
                output_name.as_ptr(),
                width,
                height,
            )
        })
    })
}

pub fn create_context() -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::create_context() }))
}

pub fn infer() -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::infer() }))
}

pub fn release_resources() -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::release_resources() }))
}

pub fn set_input(input_buffer: &mut UnifiedItem<f32>) -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::set_input(input_buffer.to_device()?) }))
}

pub fn set_output(output_buffer: &mut UnifiedItem<f32>) -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::set_output(output_buffer.device()?) }))
}

pub fn postprocess_init(
    max_detect: u16,
    conf_threshold: f32,
    iou_threshold: f32,
    feature_map_size: u16,
) -> Result<()> {
    exec_and_check(|| {
        Ok(unsafe {
            trt_op_ffi::postprocess_init(
                max_detect,
                conf_threshold,
                iou_threshold,
                feature_map_size,
            )
        })
    })
}

pub fn postprocess_init_default() -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::postprocess_init_default() }))
}

pub fn postprocess(
    input_buffer: &mut UnifiedItem<f32>,
    output_buffer: &mut UnifiedItem<f32>,
) -> Result<u16> {
    let mut num_detections = 0;
    exec_and_check(|| {
        Ok(unsafe {
            trt_op_ffi::postprocess(
                input_buffer.device()?,
                output_buffer.host(),
                &mut num_detections,
            )
        })
    })
    .map(|_| num_detections)
}

pub fn postprocess_destroy() -> Result<()> {
    exec_and_check(|| Ok(unsafe { trt_op_ffi::postprocess_destroy() }))
}

#[cfg(test)]
mod test {
    use crate::TensorBuffer;

    use super::*;

    #[test]
    fn test_infer() {
        create_engine("../model.trt", "images", "output0", 640, 480).unwrap();
        create_context().unwrap();

        let mut input = TensorBuffer::new(vec![1, 3, 640, 480]).unwrap();
        let mut output = TensorBuffer::new(vec![1, 32, 6300]).unwrap();
        set_input(&mut input).unwrap();
        set_output(&mut output).unwrap();

        infer().unwrap();

        release_resources().unwrap();
        output.to_host().unwrap();

        // for i in 4..21 {
        //     println!(
        //         "{} {}",
        //         i - 4,
        //         output.iter().skip(i * 6300).take(1).next().unwrap()
        //     );
        // }
        // for num in output.iter()
        // .enumerate()
        // .skip_while(|(_, num)| **num > 1.0)
        // .take(32)
        // {
        // println!("{} {}", idx, num);
        // assert!(num < &660.0, "num: {}", num);
        // }
        // println!(
        //     "{}",
        //     output
        //         .iter()
        //         .max_by(|a, b| a.partial_cmp(b).unwrap())
        //         .unwrap()
        // )
    }

    #[test]
    fn test_postprocess() {
        postprocess_init_default().unwrap();

        let mut input_buffer = TensorBuffer::new(vec![1, 32, 6300]).unwrap();
        let mut output_buffer = TensorBuffer::new(vec![25, 16]).unwrap();

        postprocess(&mut input_buffer, &mut output_buffer).unwrap();
        postprocess_destroy().unwrap();
    }

    #[cfg(any(target_os = "windows", target_arch = "aarch64"))]
    #[test]
    fn test_malloc() {
        let ptr: *mut f64 = cuda_malloc(1024).expect("malloc error");
        cuda_free(ptr).expect("free error");
    }

    #[cfg(any(target_os = "windows", target_arch = "aarch64"))]
    #[test]
    fn test_malloc_host() {
        let ptr: *mut u128 = cuda_malloc_host(1024).expect("malloc error");
        cuda_free_host(ptr).expect("free error");
    }

    #[cfg(not(any(target_os = "windows", target_arch = "aarch64")))]
    #[test]
    fn test_malloc_managed() {
        let ptr: *mut f32 = cuda_malloc_managed(1024).expect("malloc error");
        cuda_free(ptr).expect("free error");
    }

    #[test]
    fn test_convert_img_tensor() {
        let mut image = ImageBuffer::new(640, 480).unwrap();
        for data in image.iter_mut() {
            *data = 255;
        }
        let mut tensor = TensorBuffer::new(vec![1, 3, 640, 480]).unwrap();

        convert_rgb888_3dtensor(&mut image, &mut tensor).unwrap();
        tensor.to_host().unwrap();
        // for i in 0..3 {
        //     for data in tensor.iter().skip(640 * 640 * i).take(640 * 80) {
        //         assert_eq!(*data, 0.5);
        //     }
        //     for data in tensor.iter().skip(640 * 80 + 640 * 640 * i).take(640 * 480) {
        //         assert_eq!(*data, 1.0);
        //     }
        //     for data in tensor.iter().skip(640 * 560 + 640 * 640 * i).take(640 * 80) {
        //         assert_eq!(*data, 0.5);
        //     }
        // }
    }
}
