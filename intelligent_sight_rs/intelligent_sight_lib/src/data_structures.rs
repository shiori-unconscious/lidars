use crate::unified_item::UnifiedItem;
use anyhow::Result;
use std::ops::{Deref, DerefMut};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct DetectionBuffer {
    pub detections: Vec<Detection>,
    pub timestamp: Instant,
}

impl DetectionBuffer {
    pub fn new(len: usize) -> Self {
        DetectionBuffer {
            detections: vec![Detection::default(); len],
            timestamp: Instant::now(),
        }
    }
}

impl Deref for DetectionBuffer {
    type Target = Vec<Detection>;
    fn deref(&self) -> &Self::Target {
        &self.detections
    }
}

impl DerefMut for DetectionBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.detections
    }
}

#[derive(Debug, Clone, Default)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub conf: f32,
    pub points: [[f32; 2]; 5],
    pub cls: u32,
}

pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    data: UnifiedItem<u8>,
    pub timestamp: Instant,
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        Ok(ImageBuffer {
            width,
            height,
            data: UnifiedItem::new((width * height * 3) as usize)?, // 3 channels
            timestamp: Instant::now(),
        })
    }
}

impl Default for ImageBuffer {
    fn default() -> Self {
        match ImageBuffer::new(640, 480) {
            Ok(image) => image,
            Err(err) => {
                panic!(
                    "Failed to create default ImageBuffer, allocation failure: {}",
                    err
                );
            }
        }
    }
}

impl Deref for ImageBuffer {
    type Target = UnifiedItem<u8>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for ImageBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Clone for ImageBuffer {
    fn clone(&self) -> Self {
        let mut data = UnifiedItem::new(self.data.len()).expect("fail to malloc UnifiedItem<u8>");
        data.iter_mut()
            .zip(self.data.iter())
            .for_each(|(dst, src)| *dst = *src);
        ImageBuffer {
            width: self.width,
            height: self.height,
            data,
            timestamp: self.timestamp,
        }
    }
}

pub struct TensorBuffer {
    size: Vec<usize>,
    data: UnifiedItem<f32>,
    pub timestamp: Instant,
}

impl TensorBuffer {
    pub fn new(size: Vec<usize>) -> Result<Self> {
        Ok(TensorBuffer {
            data: UnifiedItem::new(size.iter().fold(1, |sum, num| sum * num))?,
            size,
            timestamp: Instant::now(),
        })
    }

    pub fn size(&self) -> &Vec<usize> {
        &self.size
    }

    pub fn resize(&mut self, size: Vec<usize>) {
        self.size = size;
    }
}

impl Deref for TensorBuffer {
    type Target = UnifiedItem<f32>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for TensorBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Clone for TensorBuffer {
    fn clone(&self) -> Self {
        let mut data = UnifiedItem::new(self.data.len()).expect("fail to malloc UnifiedItem<f32>");
        data.iter_mut()
            .zip(self.data.iter())
            .for_each(|(dst, src)| *dst = *src);
        TensorBuffer {
            size: self.size.clone(),
            data,
            timestamp: self.timestamp,
        }
    }
}
