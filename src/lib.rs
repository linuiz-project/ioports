#![no_std]

use core::marker::PhantomData;

mod rw;
pub use rw::*;

impl PortReadWrite for u8 {}
impl PortReadWrite for u16 {}
impl PortReadWrite for u32 {}

/// A port wrapper that only allows read operations.
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadOnlyPort<T: PortRead> {
    address: PortAddress,
    marker: PhantomData<T>,
}

impl<T: PortRead> ReadOnlyPort<T> {
    /// Creates a read-only port wrapper pointing to the provided address.
    ///
    /// ### Safety
    ///
    /// - Provided port address must point to a valid device.
    /// - Provided port address should not be otherwise aliased.
    /// - Port must be valid for reading types of size `T`.
    pub const unsafe fn new(address: PortAddress) -> Self {
        ReadOnlyPort {
            address,
            marker: PhantomData,
        }
    }

    /// The address of the port.
    pub const fn address(&self) -> PortAddress {
        self.address
    }

    /// Reads a `T` from the port.
    pub fn read(&self) -> T {
        unsafe { T::read(self.address()) }
    }
}

/// A port wrapper that only allows write operations.
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WriteOnlyPort<T: PortWrite> {
    address: PortAddress,
    marker: PhantomData<T>,
}

impl<T: PortWrite> WriteOnlyPort<T> {
    /// Creates a write-only port wrapper pointing to the provided address.
    ///
    /// ### Safety
    ///
    /// - Provided port address must point to a valid device.
    /// - Provided port address should not be otherwise aliased.
    /// - Port must be valid for reading types of size `T`.
    pub const unsafe fn new(address: PortAddress) -> Self {
        WriteOnlyPort {
            address,
            marker: PhantomData,
        }
    }

    /// The address of the port.
    pub const fn address(&self) -> PortAddress {
        self.address
    }

    /// Writes a `T` to the port.
    pub fn write(&mut self, value: T) {
        unsafe { T::write(self.address(), value) }
    }
}

/// A port wrapper that allows read and write operations.
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadWritePort<T: PortReadWrite> {
    address: PortAddress,
    marker: PhantomData<T>,
}

impl<T: PortReadWrite> ReadWritePort<T> {
    /// Creates a read-write port wrapper pointing to the provided address.
    ///
    /// ### Safety
    ///
    /// - Provided port address must point to a valid device.
    /// - Provided port address should not be otherwise aliased.
    /// - Port must be valid for reading types of size `T`.
    pub const unsafe fn new(address: PortAddress) -> Self {
        ReadWritePort {
            address,
            marker: PhantomData,
        }
    }

    /// The address of the port.
    pub const fn address(&self) -> PortAddress {
        self.address
    }

    /// Reads a `T` from the port.
    pub fn read(&self) -> T {
        unsafe { T::read(self.address()) }
    }

    /// Writes a `T` to the port.
    pub fn write(&mut self, value: T) {
        unsafe { T::write(self.address(), value) }
    }
}
