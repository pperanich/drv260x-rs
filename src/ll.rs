//! Low-level register and interface definitions for DRV260X haptic drivers

use embedded_hal::i2c::I2c;

/// I2C address of the DRV260X family
pub const I2C_ADDRESS: u8 = 0x5A;

/// Device interface error types
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DeviceInterfaceError<I2cError> {
    /// I2C communication error
    I2c(I2cError),
}

// Allow missing docs for generated device code
#[allow(missing_docs)]
mod device_generated {
    device_driver::create_device!(
        device_name: Device,
        manifest: "device.yaml"
    );
}
pub use device_generated::*;

/// Device interface implementation
#[derive(Debug)]
pub struct DeviceInterface<I2c> {
    /// The I2C interface
    pub i2c: I2c,
}

impl<I2cTrait: I2c> device_driver::RegisterInterface for DeviceInterface<I2cTrait> {
    type AddressType = u8;
    type Error = DeviceInterfaceError<I2cTrait::Error>;

    fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(I2C_ADDRESS, &[address], data)
            .map_err(DeviceInterfaceError::I2c)
    }

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buf = [0u8; 9]; // Max for multi-byte writes (address + up to 8 bytes)
        buf[0] = address;
        buf[1..1 + data.len()].copy_from_slice(data);
        self.i2c
            .write(I2C_ADDRESS, &buf[..1 + data.len()])
            .map_err(DeviceInterfaceError::I2c)
    }
}

#[cfg(feature = "async")]
impl<I2cTrait: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface
    for DeviceInterface<I2cTrait>
{
    type AddressType = u8;
    type Error = DeviceInterfaceError<I2cTrait::Error>;

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(I2C_ADDRESS, &[address], data)
            .await
            .map_err(DeviceInterfaceError::I2c)
    }

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buf = [0u8; 9]; // Max for multi-byte writes (address + up to 8 bytes)
        buf[0] = address;
        buf[1..1 + data.len()].copy_from_slice(data);
        self.i2c
            .write(I2C_ADDRESS, &buf[..1 + data.len()])
            .await
            .map_err(DeviceInterfaceError::I2c)
    }
}
