use crate::*;
use std::os::raw::c_int;
use uom::si::{electric_potential::volt, f64::*, frequency::hertz, time::second};

#[derive(Debug)]
pub struct Oscilloscope<'handle> {
    pub(crate) device_handle: c_int,
    pub(crate) phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> Oscilloscope<'handle> {
    pub fn reset(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInReset self.device_handle)
    }

    /// When zero, the record will run indefinitely.
    pub fn set_record_length(&mut self, time: Time) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInFrequencySet self.device_handle, time.get::<second>())
    }

    pub fn get_record_length(&self) -> Result<Time, WaveFormsError> {
        Ok(Time::new::<second>(
            get_float!(FDwfAnalogInRecordLengthGet self.device_handle)?,
        ))
    }

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

    pub fn max_sample_frequency(&self) -> Result<Frequency, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfAnalogInFrequencyInfo self.device_handle, &mut min, &mut max)?;
        Ok(Frequency::new::<hertz>(max))
    }

    pub fn min_sample_frequency(&self) -> Result<Frequency, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfAnalogInFrequencyInfo self.device_handle, &mut min, &mut max)?;
        Ok(Frequency::new::<hertz>(min))
    }

    pub fn adc_bit_width(&self) -> Result<u32, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfAnalogInBitsInfo self.device_handle).map(|x| u32::try_from(x).unwrap_or(0))
    }

    pub fn supported_buffer_size_range(&self) -> Result<RangeInclusive<usize>, WaveFormsError> {
        use std::convert::TryFrom;
        let mut min = 0;
        let mut max = 0;
        call!(FDwfAnalogInBufferSizeInfo self.device_handle, &mut min, &mut max)?;
        Ok(min as usize..=usize::try_from(max).unwrap_or(usize::MAX))
    }

    pub fn set_buffer_size(&mut self, size: usize) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInBufferSizeSet self.device_handle, size as c_int)
    }

    pub fn get_buffer_size(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfAnalogInBufferSizeGet self.device_handle)
            .map(|x| usize::try_from(x).unwrap_or(usize::MAX))
    }

    pub fn set_acquisition_mode(&mut self, mode: AcquisitionMode) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInAcquisitionModeSet self.device_handle, mode.into())
    }

    pub fn get_acquisition_mode(&mut self) -> Result<AcquisitionMode, WaveFormsError> {
        Ok(AcquisitionMode::from(
            get_int!(FDwfAnalogInAcquisitionModeGet self.device_handle)?,
        ))
    }

    pub fn supported_acquisition_modes(&self) -> Result<SupportedAcquisitionModes, WaveFormsError> {
        Ok(SupportedAcquisitionModes::from(
            get_int!(FDwfAnalogInAcquisitionModeInfo self.device_handle)?,
        ))
    }

    pub fn get_trigger(&self) -> Result<TriggerSource, WaveFormsError> {
        Ok(TriggerSource::from(
            get_int!(FDwfAnalogInSamplingSourceGet self.device_handle)?,
        ))
    }

    pub fn set_trigger(&mut self, src: TriggerSource) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInSamplingSourceSet self.device_handle, src.into())
    }

    pub fn get_sampling_slope(&self) -> Result<SamplingSlope, WaveFormsError> {
        Ok(SamplingSlope::from(
            get_int!(FDwfAnalogInSamplingSlopeGet self.device_handle)?,
        ))
    }

    pub fn set_sampling_slope(&mut self, slope: SamplingSlope) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInSamplingSlopeSet self.device_handle, slope.into())
    }

    pub fn set_sampling_delay(&mut self, time: Time) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInSamplingDelaySet self.device_handle, time.get::<second>())
    }

    pub fn get_sampling_delay(&self) -> Result<Time, WaveFormsError> {
        Ok(Time::new::<second>(
            get_float!(FDwfAnalogInSamplingDelayGet self.device_handle)?,
        ))
    }

    /// Check the instrument state without reading data from the device
    pub fn state(&self) -> Result<InstrumentState, WaveFormsError> {
        Ok(InstrumentState::from(
            get_int!(FDwfAnalogInStatus 0, self.device_handle)?,
        ))
    }

    pub fn channels(&mut self) -> Result<Vec<Channel<'handle>>, WaveFormsError> {
        let channel_count = get_int!(FDwfAnalogInChannelCount self.device_handle)?;
        Ok((0..channel_count)
            .map(|channel_index| Channel {
                device_handle: self.device_handle,
                index: channel_index,
                phantom: std::marker::PhantomData,
            })
            .collect::<Vec<_>>())
    }
}

pub struct Channel<'handle> {
    device_handle: c_int,
    index: c_int,
    phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> Channel<'handle> {
    pub fn enable(&mut self) -> Result<(), WaveFormsError> {
        set_true!(FDwfAnalogInChannelEnableSet self.device_handle, self.index)
    }

    pub fn disable(&mut self) -> Result<(), WaveFormsError> {
        set_false!(FDwfAnalogInChannelEnableSet self.device_handle, self.index)
    }

    pub fn is_enabled(&self) -> Result<bool, WaveFormsError> {
        get_bool!(FDwfAnalogInChannelEnableGet self.device_handle, self.index)
    }

    pub fn set_filter(&mut self, filter: Filter) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInChannelFilterSet self.device_handle, self.index, filter.into())
    }

    pub fn get_filter(&self) -> Result<Filter, WaveFormsError> {
        Ok(Filter::from(
            get_int!(FDwfAnalogInChannelFilterGet self.device_handle, self.index)?,
        ))
    }

    pub fn supported_filters(&self) -> Result<SupportedFilters, WaveFormsError> {
        Ok(SupportedFilters::from(
            get_int!(FDwfAnalogInChannelFilterInfo self.device_handle)?,
        ))
    }

    /// Voltage range steps supported by the scope
    pub fn supported_range_steps(&self) -> Result<Vec<ElectricPotential>, WaveFormsError> {
        use std::convert::TryFrom;
        let mut steps = [0.; 32];
        let mut num_steps = 0;
        unsafe {
            if FDwfAnalogInChannelRangeSteps(self.device_handle, &mut steps, &mut num_steps) == 0 {
                return Err(WaveFormsError::get());
            }
        }
        Ok((0..usize::try_from(num_steps).unwrap_or(0))
            .map(|step| ElectricPotential::new::<volt>(steps[step]))
            .collect::<Vec<_>>())
    }

    pub fn set_range(&mut self, volts: ElectricPotential) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInChannelRangeSet self.device_handle, self.index, volts.get::<volt>())
    }

    pub fn get_range(&self) -> Result<ElectricPotential, WaveFormsError> {
        Ok(ElectricPotential::new::<volt>(
            get_float!(FDwfAnalogInChannelRangeGet self.device_handle, self.index)?,
        ))
    }

    pub fn set_offset(&mut self, volts: ElectricPotential) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInChannelOffsetSet self.device_handle, self.index, volts.get::<volt>())
    }

    pub fn get_offset(&self) -> Result<ElectricPotential, WaveFormsError> {
        Ok(ElectricPotential::new::<volt>(
            get_float!(FDwfAnalogInChannelOffsetGet self.device_handle, self.index)?,
        ))
    }

    /// Informs the device of externally applied attenuation for the channel
    pub fn set_attenuation(&mut self, attenuation: f64) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInChannelAttenuationSet self.device_handle, self.index, attenuation)
    }

    pub fn get_attenuation(&self) -> Result<f64, WaveFormsError> {
        get_float!(FDwfAnalogInChannelAttenuationGet self.device_handle, self.index)
    }
}

enum_only! {
    SamplingSlope i32 {
        /// For edge and transition trigger on rising edge.
        /// For pulse trigger on positive pulse; For window exiting.
        Rise => DwfTriggerSlopeRise,
        /// For edge and transition trigger on falling edge.
        /// For pulse trigger on negative pulse; For window entering.
        Fall => DwfTriggerSlopeFall,
        /// For edge and transition trigger on either edge.
        /// For pulse trigger on either positive or negative pulse.
        Either => DwfTriggerSlopeEither
    }
}

enum_and_support_bitfield! {
    Filter i32 {
        /// Store every Nth ADC conversion, where N = ADC frequency /acquisition frequency.
        Decimate => filterDecimate,
        /// Store the average of N ADC conversions.
        Average => filterAverage,
        /// Store interleaved, the minimum and maximum values, of 2xN conversions.
        MinMax => filterMinMax
    }
}
