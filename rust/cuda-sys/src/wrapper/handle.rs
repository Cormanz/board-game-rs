use std::ptr::null_mut;

use crate::bindings::{cudaGetDeviceCount, cudaSetDevice, cudaStream_t, cudaStreamCreate, cudaStreamDestroy, cudnnCreate, cudnnDestroy, cudnnSetStream};
use crate::bindings::cudnnHandle_t;
use crate::wrapper::status::Status;

pub fn cuda_device_count() -> i32 {
    unsafe {
        let mut count = 0;
        cudaGetDeviceCount(&mut count as *mut _).unwrap();
        count
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Device(i32);

impl Device {
    pub fn new(device: i32) -> Self {
        assert!(0 <= device && device < cuda_device_count(), "Device doesn't exist {}", device);
        Device(device)
    }

    pub fn all() -> impl Iterator<Item=Self> {
        (0..cuda_device_count()).map(|i| Device::new(i))
    }

    pub fn inner(self) -> i32 {
        self.0
    }

    pub unsafe fn switch_to(self) {
        cudaSetDevice(self.0).unwrap()
    }
}

//TODO copy? clone? default stream?
#[derive(Debug)]
pub struct CudaStream {
    device: Device,
    inner: cudaStream_t,
}

impl Drop for CudaStream {
    fn drop(&mut self) {
        unsafe {
            cudaStreamDestroy(self.inner).unwrap();
        }
    }
}

impl CudaStream {
    pub fn new(device: Device) -> Self {
        unsafe {
            let mut inner = null_mut();
            device.switch_to();
            cudaStreamCreate(&mut inner as *mut _).unwrap();
            CudaStream { device, inner }
        }
    }

    pub fn device(&self) -> Device {
        self.device
    }

    pub unsafe fn inner(&self) -> cudaStream_t {
        self.inner
    }
}

#[derive(Debug)]
pub struct CudnnHandle {
    inner: cudnnHandle_t,
    stream: CudaStream,
}

impl Drop for CudnnHandle {
    fn drop(&mut self) {
        unsafe {
            self.device().switch_to();
            cudnnDestroy(self.inner).unwrap()
        }
    }
}

impl CudnnHandle {
    pub fn new(stream: CudaStream) -> Self {
        unsafe {
            let mut inner = null_mut();
            stream.device.switch_to();
            cudnnCreate(&mut inner as *mut _).unwrap();
            cudnnSetStream(inner, stream.inner()).unwrap();
            CudnnHandle { inner, stream }
        }
    }

    pub fn device(&self) -> Device {
        self.stream.device()
    }

    pub fn stream(&self) -> &CudaStream {
        &self.stream
    }

    pub unsafe fn inner(&mut self) -> cudnnHandle_t {
        self.inner
    }
}
