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
        Ok(InstrumentState::from(
            get_int!(FDwfDigitalInStatus self.device_handle, 0)?,
        ))
    }

    /// On-device clock source frequency
    pub fn internal_clock_frequency(&self) -> Result<Frequency, WaveFormsError> {
        get_float!(FDwfDigitalInInternalClockInfo self.device_handle)
            .map(|x| Frequency::new::<hertz>(x))
    }

    pub fn set_clock_source(&mut self, src: ClockSource) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalInClockSourceSet self.device_handle, src.into())
    }

    pub fn get_clock_source(&mut self) -> Result<ClockSource, WaveFormsError> {
        get_int!(FDwfDigitalInClockSourceGet self.device_handle).map(ClockSource::from)
    }

    pub fn set_clock_divider(&mut self, divider: c_uint) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalInDividerSet self.device_handle, divider)
    }

    pub fn get_clock_divider(&mut self) -> Result<c_uint, WaveFormsError> {
        get_int!(FDwfDigitalInDividerGet self.device_handle)
    }

    pub fn max_clock_divider(&self) -> Result<c_uint, WaveFormsError> {
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

    pub fn set_sample_mode(&mut self, mode: SampleMode) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalInSampleModeSet self.device_handle, mode.into())
    }

    pub fn get_sample_mode(&mut self) -> Result<SampleMode, WaveFormsError> {
        Ok(SampleMode::from(
            get_int!(FDwfDigitalInSampleModeGet self.device_handle)?,
        ))
    }

    pub fn supported_sample_modes(&self) -> Result<SupportedSampleModes, WaveFormsError> {
        Ok(SupportedSampleModes::from(
            get_int!(FDwfDigitalInSampleModeInfo self.device_handle)?,
        ))
    }

    pub fn set_acquisition_mode(&mut self, mode: AcquisitionMode) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalInAcquisitionModeSet self.device_handle, mode.into())
    }

    pub fn get_acquisition_mode(&mut self) -> Result<AcquisitionMode, WaveFormsError> {
        Ok(AcquisitionMode::from(
            get_int!(FDwfDigitalInAcquisitionModeGet self.device_handle)?,
        ))
    }

    pub fn supported_acquisition_modes(&self) -> Result<SupportedAcquisitionModes, WaveFormsError> {
        Ok(SupportedAcquisitionModes::from(
            get_int!(FDwfDigitalInAcquisitionModeInfo self.device_handle)?,
        ))
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
