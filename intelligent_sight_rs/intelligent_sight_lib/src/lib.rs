mod cam_op;
mod data_structures;
mod gpu_op;
mod shared_buffer;
mod unified_item;

pub use cam_op::FlipFlag;
pub use cam_op::{get_image, initialize_camera, uninitialize_camera};

pub use gpu_op::{
    convert_rgb888_3dtensor, create_context, create_engine, cuda_free, infer, postprocess,
    postprocess_destroy, postprocess_init, postprocess_init_default, release_resources, set_input,
    set_output, transfer_device_to_host, transfer_host_to_device,
};

#[cfg(not(any(target_os = "windows", target_arch = "aarch64")))]
pub use gpu_op::cuda_malloc_managed;

#[cfg(any(target_os = "windows", target_arch = "aarch64"))]
pub use gpu_op::{cuda_free_host, cuda_malloc, cuda_malloc_host};

pub use data_structures::{Detection, DetectionBuffer, ImageBuffer, TensorBuffer};
pub use shared_buffer::{Reader, SharedBuffer, Writer};
pub use unified_item::{UnifiedItem, UnifiedTrait};
