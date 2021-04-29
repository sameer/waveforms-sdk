use paste::paste;
use std::ffi::CStr;
use std::os::raw::*;
use uom::si::{f64::*, frequency::hertz, time::second};

mod enums;
#[cfg(test)]
mod tests;
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;
use enums::*;

#[derive(Debug)]
pub struct WaveFormsError {
    error_code: WaveFormsErrorCode,
    reason: String,
}

impl WaveFormsError {
    pub fn error_code(&self) -> &WaveFormsErrorCode {
        &self.error_code
    }

    pub fn reason(&self) -> &String {
        &self.reason
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum WaveFormsErrorCode {
    /// Unknown error reported by SDK
    Unknown,
    /// SDK could not lock an API mutex within some pre-defined time period
    ApiLockTimeout,
    /// Device is already in use by this or another program.
    AlreadyOpened,
    /// SDK call does not make sense for or is not supported by this device
    NotSupported,
    /// N-th parameter in an SDK call is invalid
    InvalidParameter(usize),
    /// Rust SDK bindings are not aware of this error code
    Other,
}

impl WaveFormsError {
    fn get() -> Self {
        Self {
            error_code: WaveFormsErrorCode::get(),
            reason: unsafe {
                let mut buffer = [0i8; 512];
                FDwfGetLastErrorMsg(&mut buffer);
                CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .to_string()
            },
        }
    }
}

impl WaveFormsErrorCode {
    fn get() -> Self {
        let mut error_code: c_int = 0;
        unsafe {
            FDwfGetLastError(&mut error_code);
        }
        use WaveFormsErrorCode::*;
        dbg!(error_code);
        match error_code {
            dwfercUnknownError => Unknown,
            dwfercApiLockTimeout => ApiLockTimeout,
            dwfercAlreadyOpened => AlreadyOpened,
            dwfercNotSupported => NotSupported,
            dwfercInvalidParameter0 => InvalidParameter(0),
            dwfercInvalidParameter1 => InvalidParameter(1),
            dwfercInvalidParameter2 => InvalidParameter(2),
            dwfercInvalidParameter3 => InvalidParameter(3),
            dwfercInvalidParameter4 => InvalidParameter(4),
            _ => Other,
        }
    }
}

macro_rules! get_string {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut buffer = [0i8; 32];
            let res = $func($($arg,)* &mut buffer);
            if res != 0 {
                Ok(CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .to_string())
            } else {
                Err(WaveFormsError::get())
            }
        }
    };
}

macro_rules! get_int {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! get_float {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0.;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! get_bool {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val != 0) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! call {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let res = $func($($arg,)*);
            if res != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident : $ty: ty),* }) => {
        #[derive(Debug, PartialEq)]
        pub struct $name {
            $(
                $field: $ty,
            )*
        }

        paste! {
            impl $name {
                $(
                    pub fn [<$field>](&self) -> &$ty {
                        &self.$field
                    }
                )*
            }
        }
    }
}

/// WaveForms SDK version
pub fn version() -> String {
    get_string!(FDwfGetVersion).unwrap()
}

#[derive(Debug)]
pub struct Device {
    index: c_int,
    ty: DeviceType,
    username: String,
    name: String,
    serial_number: String,
    configs: Vec<Config>,
}

impl Device {
    pub fn open_with_config(&self, config: &Config) -> Result<DeviceHandle, WaveFormsError> {
        // TODO: libdwf doesn't actually return the correct error
        // for this, overriding their logic here.
        if get_bool!(FDwfEnumDeviceIsOpened self.index)? {
            return Err(WaveFormsError {
                reason: "device was already opened".to_owned(),
                error_code: WaveFormsErrorCode::AlreadyOpened,
            });
        }
        let handle = get_int!(FDwfDeviceConfigOpen self.index, config.index)?;
        Ok(DeviceHandle {
            handle: Some(handle),
        })
    }

    pub fn open(&self) -> Result<DeviceHandle, WaveFormsError> {
        // TODO: libdwf doesn't actually return the correct error
        // for this, overriding their logic here.
        if get_bool!(FDwfEnumDeviceIsOpened self.index)? {
            return Err(WaveFormsError {
                reason: "device was already opened".to_owned(),
                error_code: WaveFormsErrorCode::AlreadyOpened,
            });
        }
        let handle = get_int!(FDwfDeviceOpen self.index)?;
        Ok(DeviceHandle {
            handle: Some(handle),
        })
    }
}

pub fn enumerate_devices() -> impl Iterator<Item = Device> {
    let device_count = get_int!(FDwfEnum EnumFilter::All as i32).unwrap();
    (0..device_count).map(|device_index| {
        let mut version = 0;
        let id = get_int!(FDwfEnumDeviceType device_index, &mut version).unwrap();

        let config_count = get_int!(FDwfEnumConfig device_index).unwrap();
        let configs = (0..config_count)
            .map(|config_index| Config {
                index: config_index,
                analog: DomainConfig {
                    in_channel_count:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogInChannelCount).unwrap()
                            as usize,
                    out_channel_count:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogOutChannelCount)
                            .unwrap() as usize,
                    io_channel_count:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogIOChannelCount).unwrap()
                            as usize,
                    in_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogInBufferSize).unwrap()
                            as usize,
                    out_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogOutBufferSize).unwrap()
                            as usize,
                },
                digital: DomainConfig {
                    in_channel_count:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalInChannelCount)
                            .unwrap() as usize,
                    out_channel_count:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalOutChannelCount)
                            .unwrap() as usize,
                    io_channel_count:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalIOChannelCount)
                            .unwrap() as usize,
                    in_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalInBufferSize).unwrap()
                            as usize,
                    out_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalOutBufferSize).unwrap()
                            as usize,
                },
            })
            .collect::<Vec<_>>();

        Device {
            index: device_index,
            ty: id.into(),
            username: get_string!(FDwfEnumUserName device_index).unwrap(),
            name: get_string!(FDwfEnumDeviceName device_index).unwrap(),
            serial_number: get_string!(FDwfEnumSN device_index).unwrap(),
            configs,
        }
    })
}

make_a_struct_and_getters! {
    DomainConfig {
        in_channel_count: usize,
        out_channel_count: usize,
        io_channel_count: usize,
        in_buffer_size: usize,
        out_buffer_size: usize
    }
}

make_a_struct_and_getters! {
    Config {
        index: c_int,
        analog: DomainConfig,
        digital: DomainConfig
    }
}

#[derive(Debug)]
pub struct DeviceHandle {
    handle: Option<c_int>,
}

impl DeviceHandle {
    /// Returns the supported trigger source options for the global trigger bus.
    pub fn supported_trigger_sources(&self) -> Result<SupportedTriggerSources, WaveFormsError> {
        Ok(SupportedTriggerSources::from(
            get_int!(FDwfDeviceTriggerInfo self.handle.unwrap())?,
        ))
    }

    pub fn oscilloscope<'handle>(
        &'handle mut self,
    ) -> Result<Oscilloscope<'handle>, WaveFormsError> {
        call!(FDwfAnalogInReset self.handle.unwrap())?;
        Ok(Oscilloscope {
            device_handle: self.handle.unwrap(),
            phantom: std::marker::PhantomData,
        })
    }

    /// Generates one pulse on the PC trigger line.
    pub fn trigger_pc(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfDeviceTriggerPC self.handle.unwrap())
    }

    pub fn close(mut self) -> Result<(), WaveFormsError> {
        self.close_ref()
    }

    fn close_ref(&mut self) -> Result<(), WaveFormsError> {
        if let Some(handle) = self.handle {
            self.handle = None;
            call!(FDwfDeviceClose handle)
        } else {
            Ok(())
        }
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        self.close_ref().unwrap()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct SupportedTriggerSources {
    /// From this computer
    pc: bool,

    /// From the analog in detector
    detector_analog_in: bool,

    /// From the digital in detector
    detector_digital_in: bool,

    /// When this is running
    analog_in: bool,

    /// When this is running
    digital_in: bool,

    /// When this is running
    analog_out_1: bool,

    /// When this is running
    analog_out_2: bool,

    /// When this is running
    analog_out_3: bool,

    /// When this is running
    analog_out_4: bool,

    /// From external signal
    external_1: bool,

    /// From external signal
    external_2: bool,

    /// From external signal
    external_3: bool,

    /// From external signal
    external_4: bool,

    /// Undocumented
    high: bool,

    /// Undocumented
    low: bool,
}

impl From<c_int> for SupportedTriggerSources {
    fn from(x: c_int) -> Self {
        dbg!(x);
        Self {
            pc: (x & ((1 as c_int) << trigsrcPC)) != 0,
            detector_analog_in: (x & ((1 as c_int) << trigsrcDetectorAnalogIn)) != 0,
            detector_digital_in: (x & ((1 as c_int) << trigsrcDetectorDigitalIn)) != 0,
            analog_in: (x & ((1 as c_int) << trigsrcAnalogIn)) != 0,
            digital_in: (x & ((1 as c_int) << trigsrcDigitalIn)) != 0,
            analog_out_1: (x & ((1 as c_int) << trigsrcAnalogOut1)) != 0,
            analog_out_2: (x & ((1 as c_int) << trigsrcAnalogOut2)) != 0,
            analog_out_3: (x & ((1 as c_int) << trigsrcAnalogOut3)) != 0,
            analog_out_4: (x & ((1 as c_int) << trigsrcAnalogOut4)) != 0,
            external_1: (x & ((1 as c_int) << trigsrcExternal1)) != 0,
            external_2: (x & ((1 as c_int) << trigsrcExternal2)) != 0,
            external_3: (x & ((1 as c_int) << trigsrcExternal3)) != 0,
            external_4: (x & ((1 as c_int) << trigsrcExternal4)) != 0,
            high: (x & ((1 as c_int) << trigsrcHigh)) != 0,
            low: (x & ((1 as c_int) << trigsrcLow)) != 0,
        }
    }
}

#[derive(Debug)]
pub struct Oscilloscope<'handle> {
    device_handle: c_int,
    phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> Oscilloscope<'handle> {
    pub fn reset(&self) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInReset self.device_handle)
    }

    /// Sets the Record length in seconds.With length of zero, the record will run indefinitely.
    pub fn set_record_length(&mut self, time: Time) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInFrequencySet self.device_handle, time.get::<second>())
    }

    /// Gets the current Record length
    pub fn get_record_length(&self) -> Result<Time, WaveFormsError> {
        Ok(Time::new::<second>(
            get_float!(FDwfAnalogInRecordLengthGet self.device_handle)?,
        ))
    }

    /// Sets the sample frequency for the instrument.
    pub fn set_sample_frequency(&mut self, freq: Frequency) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInFrequencySet self.device_handle, freq.get::<hertz>())
    }

    /// Reads the configured sample frequency. The AnalogIn ADC always runs at maximum frequency,
    /// but the method in which the samples are stored in the buffer can be individually configured
    /// for each channel with filters.
    pub fn get_sample_frequency(&self) -> Result<Frequency, WaveFormsError> {
        Ok(Frequency::new::<hertz>(
            get_float!(FDwfAnalogInFrequencyGet self.device_handle)?,
        ))
    }

    /// Retrieves the maximum (ADC frequency) settable sample frequency.
    pub fn max_sample_frequency(&self) -> Result<Frequency, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfAnalogInFrequencyInfo self.device_handle, &mut min, &mut max)?;
        Ok(Frequency::new::<hertz>(max))
    }

    /// Retrieves the minimum (ADC frequency) settable sample frequency.
    pub fn min_sample_frequency(&self) -> Result<Frequency, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfAnalogInFrequencyInfo self.device_handle, &mut min, &mut max)?;
        Ok(Frequency::new::<hertz>(min))
    }

    /// Retrieves the number bits used by the AnalogIn ADC.
    pub fn num_adc_bits(&self) -> Result<usize, WaveFormsError> {
        Ok(get_int!(FDwfAnalogInBitsInfo self.device_handle)? as usize)
    }
}
