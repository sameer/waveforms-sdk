use crate::*;
use std::os::raw::c_int;
use uom::si::f64::Frequency;
use uom::si::frequency::hertz;

#[derive(Debug)]
pub struct LogicAnalyzer<'handle> {
    pub(crate) device_handle: c_int,
    pub(crate) phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> LogicAnalyzer<'handle> {
    pub fn reset(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalInReset self.device_handle)
    }

    pub fn start(&mut self) -> Result<(), WaveFormsError> {
        set_true!(FDwfDigitalInConfigure self.device_handle, 0)
    }

    pub fn stop(&mut self) -> Result<(), WaveFormsError> {
        set_false!(FDwfDigitalInConfigure self.device_handle, 0)
    }

    pub fn state(&self) -> Result<InstrumentState, WaveFormsError> {
        use core::convert::TryFrom;
        get_int!(FDwfDigitalInStatus self.device_handle, 0).and_then(InstrumentState::try_from)
    }

    /// On-device clock source frequency
    pub fn internal_clock_frequency(&self) -> Result<Frequency, WaveFormsError> {
        get_float!(FDwfDigitalInInternalClockInfo self.device_handle)
            .map(|x| Frequency::new::<hertz>(x))
    }

    enum_getter_and_setter! {
        clock_source ClockSource FDwfDigitalInClockSource device_handle
    }

    int_getter_and_setter! {
        clock_divider u32 FDwfDigitalInDivider device_handle
    }

    pub fn max_clock_divider(&self) -> Result<u32, WaveFormsError> {
        Ok(get_int!(FDwfDigitalInDividerInfo self.device_handle)?)
    }

    pub fn bit_width(&self) -> Result<u32, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfDigitalInBitsInfo self.device_handle).map(|x| u32::try_from(x).unwrap_or(0))
    }

    pub fn max_buffer_size(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfDigitalInBufferSizeInfo self.device_handle)
            .map(|x| usize::try_from(x).unwrap_or(usize::MAX))
    }

    pub fn set_buffer_size(&mut self, size: usize) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalInBufferSizeSet self.device_handle, size as c_int)
    }

    pub fn get_buffer_size(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfDigitalInBufferSizeGet self.device_handle)
            .map(|x| usize::try_from(x).unwrap_or(usize::MAX))
    }

    enum_getter_and_setter! {
        sample_mode SampleMode FDwfDigitalInSampleMode device_handle
    }

    pub fn sample_modes(&self) -> Result<SupportedSampleModes, WaveFormsError> {
        get_int!(FDwfDigitalInSampleModeInfo self.device_handle).map(SupportedSampleModes::from)
    }

    enum_getter_and_setter! {
        acquisition_mode AcquisitionMode FDwfDigitalInAcquisitionMode device_handle
    }

    pub fn acquisition_modes(&self) -> Result<SupportedAcquisitionModes, WaveFormsError> {
        get_int!(FDwfDigitalInAcquisitionModeInfo self.device_handle)
            .map(SupportedAcquisitionModes::from)
    }
}

enum_and_support_bitfield! {
    ClockSource c_int {
        Internal => DwfDigitalInClockSourceInternal,
        External => DwfDigitalInClockSourceExternal,
        External2 => DwfDigitalInClockSourceExternal2
    }
}

enum_and_support_bitfield! {
    SampleMode c_int {
        Simple => DwfDigitalInSampleModeSimple,
        Noise => DwfDigitalInSampleModeNoise
    }
}
