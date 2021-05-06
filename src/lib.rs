use paste::paste;
use std::ffi::CStr;
use std::ops::RangeInclusive;
use std::os::raw::*;

#[cfg(test)]
mod tests;

#[macro_use]
mod macros;

/// Analog input, output, and I/O
pub mod analog;
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
/// Digital input, output, and protocols
pub mod digital;

use analog::{gen::WaveformGenerator, scope::Oscilloscope};
use bindings::*;
use digital::{analyzer::LogicAnalyzer, gen::PatternGenerator, protocols::Protocols};

#[derive(Debug)]
/// Any error returned by the wrapped WaveForms SDK. Includes a descriptive reason.
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
    ///
    /// The Rust bindings are most likely to blame for this
    InvalidParameter(u8),
    /// Rust SDK bindings are not aware of this error code
    Other,
    /// WaveForms SDK returned an unknown enum variant.
    /// 
    /// This can happen if the Rust SDK bindings are not up to date with the latest
    /// version of WaveForms SDK.
    UnknownVariant,
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

/// WaveForms SDK version (i.e. `3.16.3`)
///
/// See [download page](https://reference.digilentinc.com/reference/software/waveforms/waveforms-3/start) for the latest version.
pub fn version() -> String {
    get_string!(FDwfGetVersion).unwrap()
}

/// Discovered with [iter_devices]
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

    /// Acquires an exclusive lock on the device
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

/// Detect and iterate over found [Device]s
pub fn iter_devices() -> impl Iterator<Item = Device> {
    use core::convert::TryFrom;
    let device_count = get_int!(FDwfEnum DetectFilter::All.into()).unwrap();
    (0..device_count).map(|device_index| {
        let mut version = 0;
        let id = get_int!(FDwfEnumDeviceType device_index, &mut version).unwrap();

        let config_count = get_int!(FDwfEnumConfig device_index).unwrap();
        let configs = (0..config_count)
            .map(|config_index| Config {
                index: config_index,
                analog: DomainConfig {
                    input_channels:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogInChannelCount).unwrap()
                            as u32,
                    output_channels:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogOutChannelCount)
                            .unwrap() as u32,
                    io_channels: get_int!(FDwfEnumConfigInfo config_index, DECIAnalogIOChannelCount)
                        .unwrap() as u32,
                    input_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogInBufferSize).unwrap()
                            as u32,
                    output_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIAnalogOutBufferSize).unwrap()
                            as u32,
                },
                digital: DomainConfig {
                    input_channels:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalInChannelCount)
                            .unwrap() as u32,
                    output_channels:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalOutChannelCount)
                            .unwrap() as u32,
                    io_channels:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalIOChannelCount)
                            .unwrap() as u32,
                    input_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalInBufferSize).unwrap()
                            as u32,
                    output_buffer_size:
                        get_int!(FDwfEnumConfigInfo config_index, DECIDigitalOutBufferSize).unwrap()
                            as u32,
                },
            })
            .collect::<Vec<_>>();

        Device {
            index: device_index,
            ty: DeviceType::try_from(id).unwrap(),
            username: get_string!(FDwfEnumUserName device_index).unwrap(),
            name: get_string!(FDwfEnumDeviceName device_index).unwrap(),
            serial_number: get_string!(FDwfEnumSN device_index).unwrap(),
            configs,
        }
    })
}

enum_only! {
    /// Filter for [iter_devices] to look for a specific [DeviceType]
    DetectFilter c_int {
        All => enumfilterAll,
        ElectronicsExplorer => enumfilterEExplorer,
        AnalogDiscovery => enumfilterDiscovery,
        AnalogDiscovery2 => enumfilterDiscovery2,
        DigitalDiscovery => enumfilterDDiscovery
    }
}

enum_only! {
    DeviceType c_int {
        ElectronicsExplorer => devidEExplorer,
        AnalogDiscovery => devidDiscovery,
        AnalogDiscovery2 => devidDiscovery2,
        DigitalDiscovery => devidDDiscovery,
        AnalogDiscoveryPro => devidADP3X50
    }
}

make_struct! {
    /// Device configuration for a particular domain (analog/digital)
    DomainConfig {
        input_channels: u32,
        output_channels: u32,
        io_channels: u32,
        input_buffer_size: u32,
        output_buffer_size: u32
    }
}

make_struct! {
    /// Device configuration for all domains
    Config {
        index: c_int,
        analog: DomainConfig,
        digital: DomainConfig
    }
}

#[derive(Debug)]
/// Exclusive lock on a device
pub struct DeviceHandle {
    handle: Option<c_int>,
}

impl DeviceHandle {
    /// Returns the supported trigger source options for the global trigger bus.
    pub fn trigger_sources(&self) -> Result<SupportedTriggerSources, WaveFormsError> {
        Ok(SupportedTriggerSources::from(
            get_int!(FDwfDeviceTriggerInfo self.handle.unwrap())?,
        ))
    }

    pub fn get_trigger(&self, pin_index: u32) -> Result<TriggerSource, WaveFormsError> {
        use core::convert::TryFrom;
        get_int!(FDwfDeviceTriggerGet self.handle.unwrap(), pin_index as c_int).and_then(TriggerSource::try_from)
    }

    pub fn set_trigger(
        &mut self,
        pin_index: u32,
        src: TriggerSource,
    ) -> Result<(), WaveFormsError> {
        call!(FDwfDeviceTriggerSet self.handle.unwrap(), pin_index as c_int, src.into())
    }

    /// Generate one pulse on the PC trigger line.
    ///
    /// This can be used to trigger multiple instruments synchronously.
    pub fn trigger_pc(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfDeviceTriggerPC self.handle.unwrap())
    }

    /// Analog in
    pub fn oscilloscope<'handle>(
        &'handle mut self,
    ) -> Result<Oscilloscope<'handle>, WaveFormsError> {
        Ok(Oscilloscope {
            device_handle: self.handle.unwrap(),
            phantom: std::marker::PhantomData,
        })
    }

    /// Analog out
    pub fn waveform_generator<'handle>(
        &'handle mut self,
    ) -> Result<WaveformGenerator<'handle>, WaveFormsError> {
        Ok(WaveformGenerator {
            device_handle: self.handle.unwrap(),
            phantom: std::marker::PhantomData,
        })
    }

    /// Digital in
    pub fn logic_analyzer<'handle>(
        &'handle mut self,
    ) -> Result<LogicAnalyzer<'handle>, WaveFormsError> {
        Ok(LogicAnalyzer {
            device_handle: self.handle.unwrap(),
            phantom: std::marker::PhantomData,
        })
    }

    /// Digital out
    pub fn pattern_generator<'handle>(
        &'handle mut self,
    ) -> Result<PatternGenerator<'handle>, WaveFormsError> {
        Ok(PatternGenerator {
            device_handle: self.handle.unwrap(),
            phantom: std::marker::PhantomData,
        })
    }

    /// Digital I/O
    pub fn protocols<'handle>(&'handle mut self) -> Result<Protocols<'handle>, WaveFormsError> {
        Ok(Protocols {
            device_handle: self.handle.unwrap(),
            phantom: std::marker::PhantomData,
        })
    }

    /// Close the handle when you are done using the device.
    ///
    /// This will be done on your behalf when the handle is dropped.
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

enum_and_support_bitfield! {
    /// Sources for the on-device global trigger bus.
    TriggerSource c_uchar {
        None => trigsrcNone,
        /// From this computer
        Pc => trigsrcPC,
        /// From the analog in detector
        DetectorAnalogIn => trigsrcDetectorAnalogIn,
        /// From the digital in detector
        DetectorDigitalIn => trigsrcDetectorDigitalIn,
        /// When this is running
        AnalogIn => trigsrcAnalogIn,
        /// When this is running
        DigitalIn => trigsrcDigitalIn,
        /// When this is running
        AnalogOut1 => trigsrcAnalogOut1,
        /// When this is running
        AnalogOut2 => trigsrcAnalogOut2,
        /// When this is running
        AnalogOut3 => trigsrcAnalogOut3,
        /// When this is running
        AnalogOut4 => trigsrcAnalogOut4,
        /// From external signal
        External => trigsrcExternal1,
        /// From external signal
        External2 => trigsrcExternal2,
        /// From external signal
        External3 => trigsrcExternal3,
        /// From external signal
        External4 => trigsrcExternal4,
        /// Undocumented
        High => trigsrcHigh,
        /// Undocumented
        Low => trigsrcLow
    }
}

enum_and_support_bitfield! {
    /// Ways an [Oscilloscope] or [LogicAnalyzer] can acquire samples
    AcquisitionMode c_int {
        /// Perform a single buffer acquisition and rearm the instrument.
        ///
        /// The next capture will occur after data is fetched from the device.
        /// See [Oscilloscope::fetch] and [LogicAnalyzer::fetch]
        ///
        /// This is the default setting.
        Single => acqmodeSingle,
        /// Perform a continuous acquisition in FIFO style.
        ///
        /// The trigger setting is ignored. The last sample is at the end of buffer.
        ScanShift => acqmodeScanShift,
        /// Perform continuous acquisition circularly writing samples into the buffer.
        ///
        /// The trigger setting is ignored. The IndexWrite shows the buffer write position. This is similar to a heart monitor display.
        ScanScreen => acqmodeScanScreen,
        /// Perform acquisition for defined record length. See [Oscilloscope::set_record_length] and [LogicAnalyzer::set_record_length]
        Record => acqmodeRecord,
        Overs => acqmodeOvers,
        /// Perform a single buffer acquisition without rearming the instrument.
        SingleWithoutRearm => acqmodeSingle1
    }
}

enum_only! {
    /// Possible states for all instruments.Each has a different state lifecycle.
    ///
    /// # State Diagrams
    ///
    /// Components in rectangles are transitions whose destination depends on instrument configuration.
    ///
    /// Click on the diagram to go to the instrument documentation.
    /// ## Oscilloscope
    /// [![](https://reference.digilentinc.com/_media/waveforms/analog1.png?w=600&tok=e81729)](Oscilloscope)
    ///
    /// ## Waveform Generator
    /// [![](https://reference.digilentinc.com/_media/waveforms/6p1.png?w=600&tok=012218)](WaveformGenerator)
    ///
    /// ## Logic Analyzer
    /// [![](https://reference.digilentinc.com/_media/waveforms/9p1.png?w=600&tok=aca9e9)](LogicAnalyzer)
    ///
    /// ## Pattern Generator
    /// [![](https://reference.digilentinc.com/_media/waveforms/10p1.png?w=600&tok=55b446)](PatternGenerator)
    InstrumentState c_uchar {
        /// Initial state.
        Ready => DwfStateReady,
        /// Instrument is waiting to be triggered.
        Armed => DwfStateArmed,
        /// Final state after the instrument has finished running.
        Done => DwfStateDone,
        /// Instrument has been triggered and is running.
        ///
        /// For [WaveformGenerator] and [PatternGenerator],
        /// a repeat count can be set so that the instrument will
        /// run repeatedly. See [WaveformGenerator::get_repeat] or [PatternGenerator::get_repeat].
        /// These instruments will enter [InstrumentState::Armed] or [InstrumentState::Wait]
        /// depending on whether the trigger is treated as part of the repeat cycle.
        /// See [WaveformGenerator::get_repeat_includes_trigger] or [PatternGenerator::get_repeat_includes_trigger]
        Running => DwfStateRunning,
        /// Instrument is being configured.
        /// Only relevant to [Oscilloscope] and [LogicAnalyzer].
        ///
        /// The instrument can then transition back to [InstruemntState::Ready]
        /// or directly enter [InstrumentState::Prefill].
        Config => DwfStateConfig,
        /// Prefill buffer with samples needed before a trigger can occur.
        /// Only relevant to [Oscilloscope] and [LogicAnalyzer].
        Prefill => DwfStatePrefill,
        /// Instrument is waiting for the specified time.
        /// Only relevant to [WaveformGenerator] and [PatternGenerator].
        ///
        /// See [WaveformGenerator::get_wait] or [PatternGenerator::get_wait].
        Wait => DwfStateWait
    }
}
