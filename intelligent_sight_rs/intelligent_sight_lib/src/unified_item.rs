use anyhow::Result;

#[cfg(any(target_os = "windows", target_arch = "aarch64"))]
pub use unified_item_tradition::*;

#[cfg(not(any(target_os = "windows", target_arch = "aarch64")))]
// pub use unified_item_uma::*;

pub trait UnifiedTrait<T> {
    fn to_device(&mut self) -> Result<*mut T>;
    fn to_host(&mut self) -> Result<*mut T>;
    fn device(&mut self) -> Result<*mut T>;
    fn host(&mut self) -> *mut T;
    fn len(&self) -> usize;
}

#[cfg(any(target_os = "windows", target_arch = "aarch64"))]
mod unified_item_tradition {
    use super::UnifiedTrait;
    use crate::{
        cuda_free, cuda_free_host, cuda_malloc, cuda_malloc_host, gpu_op::transfer_device_to_host,
        transfer_host_to_device,
    };
    use anyhow::Result;
    use std::ops::{Deref, DerefMut};

    pub struct UnifiedItem<T> {
        device_array: Option<DeviceArray<T>>,
        host_array: HostArray<T>,
        size: usize,
    }

    impl<T> UnifiedItem<T> {
        pub fn new(size: usize) -> Result<Self>
        where
            T: Default + Copy,
        {
            Ok(UnifiedItem {
                device_array: None,
                host_array: HostArray::new(size)?,
                size,
            })
        }
    }

    impl<T> Deref for UnifiedItem<T> {
        type Target = HostArray<T>;
        fn deref(&self) -> &Self::Target {
            &self.host_array
        }
    }

    impl<T> DerefMut for UnifiedItem<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.host_array
        }
    }

    impl<T> UnifiedTrait<T> for UnifiedItem<T> {
        fn to_device(&mut self) -> Result<*mut T> {
            if self.device_array.is_none() {
                self.device_array = Some(DeviceArray::new(self.len())?);
            }
            transfer_host_to_device(
                self.host_array.as_ptr(),
                self.device_array.as_ref().unwrap().as_mut_ptr(),
                self.len(),
            )?;
            Ok(self.device_array.as_ref().unwrap().as_mut_ptr())
        }

        fn to_host(&mut self) -> Result<*mut T> {
            if self.device_array.is_none() {
                return Ok(self.host_array.as_mut_ptr());
            }
            transfer_device_to_host(
                self.host_array.as_mut_ptr(),
                self.device_array.as_ref().unwrap().as_ptr(),
                self.len(),
            )?;
            Ok(self.host_array.as_mut_ptr())
        }

        fn device(&mut self) -> Result<*mut T> {
            if self.device_array.is_none() {
                self.to_device()
            } else {
                Ok(self.device_array.as_ref().unwrap().as_mut_ptr())
            }
        }

        fn host(&mut self) -> *mut T {
            self.host_array.as_mut_ptr()
        }

        fn len(&self) -> usize {
            self.size
        }
    }

    pub struct HostArray<T> {
        size: usize,
        ptr: *mut T,
    }

    impl<T> Drop for HostArray<T> {
        fn drop(&mut self) {
            cuda_free_host(self.ptr).expect("Failed to free host memory");
        }
    }

    unsafe impl<T> Send for HostArray<T> {}

    impl<T> HostArray<T> {
        pub fn new(size: usize) -> Result<Self> {
            Ok(HostArray {
                ptr: cuda_malloc_host(size)?,
                size,
            })
        }

        pub fn len(&self) -> usize {
            self.size
        }

        pub fn iter(&self) -> HostArrayIter<'_, T> {
            HostArrayIter {
                ptr: self.ptr as *const T,
                index: 0,
                size: self.size,
                _marker: std::marker::PhantomData,
            }
        }

        pub fn iter_mut(&mut self) -> HostArrayIterMut<'_, T> {
            HostArrayIterMut {
                ptr: self.ptr,
                index: 0,
                size: self.size,
                _marker: std::marker::PhantomData,
            }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr as *const T
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            self.ptr
        }
    }

    pub struct HostArrayIter<'a, T> {
        ptr: *const T,
        index: usize,
        size: usize,
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Iterator for HostArrayIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.size {
                return None;
            }
            let ret = unsafe { self.ptr.add(self.index).as_ref() };
            self.index += 1;
            ret
        }
    }

    pub struct HostArrayIterMut<'a, T> {
        ptr: *mut T,
        index: usize,
        size: usize,
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Iterator for HostArrayIterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.size {
                return None;
            }
            let ret = unsafe { self.ptr.add(self.index).as_mut() };
            self.index += 1;
            ret
        }
    }

    pub struct DeviceArray<T> {
        #[allow(unused)]
        size: usize,
        ptr: *mut T,
    }

    unsafe impl<T> Send for DeviceArray<T> {}

    impl<T> Drop for DeviceArray<T> {
        fn drop(&mut self) {
            cuda_free(self.ptr).expect("Failed to free device memory");
        }
    }

    impl<T> DeviceArray<T> {
        pub fn new(size: usize) -> Result<Self> {
            Ok(DeviceArray {
                ptr: cuda_malloc(size)?,
                size,
            })
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }

        fn as_mut_ptr(&self) -> *mut T {
            self.ptr
        }
    }
}

#[cfg(not(any(target_os = "windows", target_arch = "aarch64")))]
mod unified_item_uma {
    use super::UnifiedTrait;
    use crate::{cuda_free, cuda_malloc_managed};
    use anyhow::Result;
    use std::ops::{Deref, DerefMut};

    pub struct UnifiedItem<T>(ManagedArray<T>);

    impl<T> UnifiedItem<T> {
        pub fn new(size: usize) -> Result<Self>
        where
            T: Default + Copy,
        {
            Ok(UnifiedItem(ManagedArray::new(size)?))
        }
    }

    impl<T> Deref for UnifiedItem<T> {
        type Target = ManagedArray<T>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for UnifiedItem<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> UnifiedTrait<T> for UnifiedItem<T> {
        fn to_device(&mut self) -> Result<*mut T> {
            Ok(self.0.as_mut_ptr())
        }

        fn to_host(&mut self) -> Result<*mut T> {
            Ok(self.0.as_mut_ptr())
        }

        fn device(&mut self) -> Result<*mut T> {
            Ok(self.0.as_mut_ptr())
        }

        fn host(&mut self) -> *mut T {
            self.0.as_mut_ptr()
        }

        fn len(&self) -> usize {
            self.0.len()
        }
    }

    pub struct ManagedArray<T> {
        ptr: *mut T,
        size: usize,
    }

    unsafe impl<T> Send for ManagedArray<T> {}

    impl<T> Clone for ManagedArray<T> {
        fn clone(&self) -> Self {
            ManagedArray {
                ptr: self.ptr,
                size: self.size,
            }
        }
    }

    impl<T> Drop for ManagedArray<T> {
        fn drop(&mut self) {
            cuda_free(self.ptr).expect("Failed to free uniform memory");
        }
    }

    impl<T> ManagedArray<T> {
        pub fn new(size: usize) -> Result<Self> {
            Ok(ManagedArray {
                size,
                ptr: cuda_malloc_managed(size)?,
            })
        }
        pub fn from_raw_parts(ptr: *mut T, size: usize) -> Self {
            ManagedArray { ptr, size }
        }

        pub fn len(&self) -> usize {
            self.size
        }

        pub fn iter(&self) -> ManagedArrayIter<'_, T> {
            ManagedArrayIter {
                ptr: self.ptr as *const T,
                index: 0,
                size: self.size,
                _marker: std::marker::PhantomData,
            }
        }

        pub fn iter_mut(&mut self) -> ManagedArrayIterMut<'_, T> {
            ManagedArrayIterMut {
                ptr: self.ptr,
                index: 0,
                size: self.size,
                _marker: std::marker::PhantomData,
            }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr as *const T
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            self.ptr
        }
    }

    pub struct ManagedArrayIter<'a, T> {
        ptr: *const T,
        index: usize,
        size: usize,
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Iterator for ManagedArrayIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.size {
                return None;
            }
            let ret = unsafe { self.ptr.add(self.index).as_ref() };
            self.index += 1;
            ret
        }
    }

    pub struct ManagedArrayIterMut<'a, T> {
        ptr: *mut T,
        index: usize,
        size: usize,
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Iterator for ManagedArrayIterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.size {
                return None;
            }
            let ret = unsafe { self.ptr.add(self.index).as_mut() };
            self.index += 1;
            ret
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unified_item_create() {
        let mut item: UnifiedItem<f64> = UnifiedItem::new(10).unwrap();
        item.iter_mut().for_each(|num| *num = 1.0);
        item.iter().for_each(|num| assert_eq!(*num, 1.0));
    }
}
