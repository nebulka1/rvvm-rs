use rvvm_sys::RVVM_DEFAULT_MEMBASE;

use crate::{
    error::InstanceCreateError,
    instance::Instance,
};

#[derive(Debug, Clone)]
pub struct InstanceBuilder {
    pub harts: usize,

    pub mem_base: u64,
    pub mem_size: usize,

    pub rv64: bool,
}

impl InstanceBuilder {
    pub fn try_build(self) -> Result<Instance, InstanceCreateError> {
        Instance::try_new(
            self.harts,
            self.mem_base,
            self.mem_size,
            self.rv64,
        )
    }

    pub fn build(self) -> Instance {
        Instance::new(self.harts, self.mem_base, self.mem_size, self.rv64)
    }
}

impl InstanceBuilder {
    pub fn mem_base(mut self, base: u64) -> Self {
        self.mem_base = base;
        self
    }

    pub fn mem_size(mut self, size: usize) -> Self {
        self.mem_size = size;
        self
    }

    pub fn rv64(mut self) -> Self {
        self.rv64 = true;
        self
    }

    pub fn harts(mut self, harts: usize) -> Self {
        self.harts = harts;
        self
    }
}

impl Default for InstanceBuilder {
    fn default() -> Self {
        Self {
            harts: 1,
            mem_base: RVVM_DEFAULT_MEMBASE as _,
            mem_size: 4096,
            rv64: false,
        }
    }
}
