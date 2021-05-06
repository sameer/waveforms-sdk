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

    pub fn start(&mut self) -> Result<(), WaveFormsError> {
        set_true!(FDwfAnalogInConfigure self.device_handle, 0)
    }

    pub fn stop(&mut self) -> Result<(), WaveFormsError> {
        set_false!(FDwfAnalogInConfigure self.device_handle, 0)
    }

    /// Check the instrument state without reading data from the device
    pub fn state(&self) -> Result<InstrumentState, WaveFormsError> {
        use core::convert::TryFrom;
        get_int!(FDwfAnalogInStatus self.device_handle, 0).and_then(InstrumentState::try_from)
    }

    /// Fetch data from the device and check the instrument state
    ///
    /// Samples are read at the `Channel` level.
    pub fn fetch(&mut self) -> Result<InstrumentState, WaveFormsError> {
        use core::convert::TryFrom;
        get_int!(FDwfAnalogInStatus self.device_handle, 1).and_then(InstrumentState::try_from)
    }

    uom_getter_and_setter! {
        /// When zero, the record will run indefinitely.
        record_length Time<second> FDwfAnalogInRecordLength device_handle
    }

    uom_getter_and_setter! {
        /// Read the configured sample frequency. The AnalogIn ADC always runs at maximum frequency,
        /// but the method in which the samples are stored in the buffer can be individually configured
        /// for each channel with filters.
        sampling_frequency Frequency<hertz> FDwfAnalogInFrequency device_handle
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

    pub fn sample_buffer_size_range(&self) -> Result<RangeInclusive<usize>, WaveFormsError> {
        use std::convert::TryFrom;
        let mut min = 0;
        let mut max = 0;
        call!(FDwfAnalogInBufferSizeInfo self.device_handle, &mut min, &mut max)?;
        Ok(min as usize..=usize::try_from(max).unwrap_or(usize::MAX))
    }

    pub fn set_sample_buffer_size(&mut self, size: usize) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInBufferSizeSet self.device_handle, size as c_int)
    }

    pub fn get_sample_buffer_size(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfAnalogInBufferSizeGet self.device_handle)
            .map(|x| usize::try_from(x).unwrap_or(usize::MAX))
    }

    pub fn max_noise_buffer_size(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfAnalogInNoiseSizeInfo self.device_handle)
            .map(|max| usize::try_from(max).unwrap_or(usize::MAX))
    }

    /// In practice, this is automatically determined by the set sample buffer size
    pub fn get_noise_buffer_size(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfAnalogInNoiseSizeGet self.device_handle)
            .map(|x| usize::try_from(x).unwrap_or(usize::MAX))
    }

    enum_getter_and_setter! {
        acquisition_mode AcquisitionMode FDwfAnalogInAcquisitionMode device_handle
    }

    pub fn acquisition_modes(&self) -> Result<SupportedAcquisitionModes, WaveFormsError> {
        get_int!(FDwfAnalogInAcquisitionModeInfo self.device_handle)
            .map(SupportedAcquisitionModes::from)
    }

    enum_getter_and_setter! {
        sampling_source TriggerSource FDwfAnalogInSamplingSource device_handle
    }

    enum_getter_and_setter! {
        sampling_slope SamplingSlope FDwfAnalogInSamplingSlope device_handle
    }

    uom_getter_and_setter! {
        sampling_delay Time<second> FDwfAnalogInSamplingDelay device_handle
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

    pub fn trigger_sources(&self) -> Result<SupportedTriggerTypes, WaveFormsError> {
        get_int!(FDwfAnalogInTriggerTypeInfo self.device_handle).map(SupportedTriggerTypes::from)
    }

    enum_getter_and_setter! {
        trigger_source TriggerSource FDwfAnalogInTriggerSource device_handle
    }

    pub fn trigger_positions(&self) -> Result<Steps<Time>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInTriggerPositionInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: Time::new::<second>(min),
            max: Time::new::<second>(max),
            num_steps: num_steps as usize,
        })
    }

    uom_getter_and_setter! {
        trigger_position Time<second> FDwfAnalogInTriggerPosition device_handle
    }

    pub fn trigger_auto_timeouts(&self) -> Result<Steps<Time>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInTriggerAutoTimeoutInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: Time::new::<second>(min),
            max: Time::new::<second>(max),
            num_steps: num_steps as usize,
        })
    }

    uom_getter_and_setter! {
        trigger_auto_timeout Time<second> FDwfAnalogInTriggerAutoTimeout device_handle
    }

    pub fn trigger_holdoffs(&self) -> Result<Steps<Time>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInTriggerHoldOffInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: Time::new::<second>(min),
            max: Time::new::<second>(max),
            num_steps: num_steps as usize,
        })
    }
    uom_getter_and_setter! {
        trigger_holdoff Time<second> FDwfAnalogInTriggerHoldOff device_handle
    }

    pub fn trigger_types(&self) -> Result<SupportedTriggerTypes, WaveFormsError> {
        get_int!(FDwfAnalogInTriggerTypeInfo self.device_handle).map(SupportedTriggerTypes::from)
    }

    enum_getter_and_setter! {
        trigger_type TriggerType FDwfAnalogInTriggerType device_handle
    }

    enum_getter_and_setter! {
        trigger_filter Filter FDwfAnalogInTriggerFilter device_handle
    }

    pub fn trigger_filters(&self) -> Result<SupportedFilters, WaveFormsError> {
        get_int!(FDwfAnalogInTriggerFilterInfo self.device_handle).map(SupportedFilters::from)
    }

    enum_getter_and_setter! {
        trigger_condition SamplingSlope FDwfAnalogInTriggerCondition device_handle
    }

    pub fn trigger_conditions(&self) -> Result<SupportedSamplingSlopes, WaveFormsError> {
        get_int!(FDwfAnalogInTriggerConditionInfo self.device_handle)
            .map(SupportedSamplingSlopes::from)
    }

    pub fn trigger_level_steps(&self) -> Result<Steps<ElectricPotential>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInTriggerLevelInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: ElectricPotential::new::<volt>(min),
            max: ElectricPotential::new::<volt>(max),
            num_steps: num_steps as usize,
        })
    }

    uom_getter_and_setter! {
        trigger_level ElectricPotential<volt> FDwfAnalogInTriggerLevel device_handle
    }

    pub fn trigger_hysteresis_steps(&self) -> Result<Steps<ElectricPotential>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInTriggerHysteresisInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: ElectricPotential::new::<volt>(min),
            max: ElectricPotential::new::<volt>(max),
            num_steps: num_steps as usize,
        })
    }

    uom_getter_and_setter! {
        trigger_hysteresis ElectricPotential<volt> FDwfAnalogInTriggerHysteresis device_handle
    }

    enum_getter_and_setter! {
        trigger_length_condition TriggerLength FDwfAnalogInTriggerLengthCondition device_handle
    }

    pub fn trigger_length_conditions(&self) -> Result<SupportedTriggerLengths, WaveFormsError> {
        get_int!(FDwfAnalogInTriggerLengthConditionInfo self.device_handle)
            .map(SupportedTriggerLengths::from)
    }

    uom_getter_and_setter! {
        trigger_length Time<second> FDwfAnalogInTriggerLength device_handle
    }

    pub fn trigger_lengths(&self) -> Result<Steps<Time>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInTriggerLengthInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: Time::new::<second>(min),
            max: Time::new::<second>(max),
            num_steps: num_steps as usize,
        })
    }
}

#[derive(Debug)]
pub struct Steps<T>
where
    T: core::fmt::Debug,
{
    pub min: T,
    pub max: T,
    // This stepping may or may not be linear
    pub num_steps: usize,
}

enum_and_support_bitfield! {
    TriggerType i32 {
        Edge => trigtypeEdge,
        Pulse => trigtypePulse,
        Transition => trigtypeTransition,
        Window => trigtypeWindow
    }
}

enum_and_support_bitfield! {
    TriggerCondition i32 {
        Edge => trigtypeEdge,
        Pulse => trigtypePulse,
        Transition => trigtypeTransition,
        Window => trigtypeWindow
    }
}

enum_and_support_bitfield! {
    TriggerLength i32 {
        Less => triglenLess,
        Timeout => triglenTimeout,
        More => triglenMore
    }
}

pub struct Samples {}

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

    enum_getter_and_setter! {
        filter Filter FDwfAnalogInChannelFilter device_handle, index
    }

    pub fn filters(&self) -> Result<SupportedFilters, WaveFormsError> {
        get_int!(FDwfAnalogInChannelFilterInfo self.device_handle).map(SupportedFilters::from)
    }

    /// Voltage range steps supported by the scope
    /// Scope will have voltage axis limits of `(+/- range / 2) - offset`
    pub fn range_steps(&self) -> Result<Steps<ElectricPotential>, WaveFormsError> {
        // use std::convert::TryFrom;
        // let mut steps = [0.; 32];
        // let mut num_steps = 0;
        // unsafe {
        //     if FDwfAnalogInChannelRangeSteps(self.device_handle, &mut steps, &mut num_steps) == 0 {
        //         return Err(WaveFormsError::get());
        //     }
        // }
        // Ok((0..usize::try_from(num_steps).unwrap_or(0))
        // .map(|step| ElectricPotential::new::<volt>(steps[step]))
        // .collect::<Vec<_>>())

        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInChannelRangeInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: ElectricPotential::new::<volt>(min),
            max: ElectricPotential::new::<volt>(max),
            num_steps: num_steps as usize,
        })
    }

    uom_getter_and_setter! {
        range ElectricPotential<volt> FDwfAnalogInChannelRange device_handle, index
    }

    /// Voltage offset steps supported by the scope
    pub fn offset_steps(&self) -> Result<Steps<ElectricPotential>, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        let mut num_steps = 0.;
        call!(FDwfAnalogInChannelOffsetInfo self.device_handle, &mut min, &mut max, &mut num_steps)?;
        Ok(Steps {
            min: ElectricPotential::new::<volt>(min),
            max: ElectricPotential::new::<volt>(max),
            num_steps: num_steps as usize,
        })
    }

    uom_getter_and_setter! {
        offset ElectricPotential<volt> FDwfAnalogInChannelOffset device_handle, index
    }

    /// Informs the device of externally applied attenuation for the channel
    pub fn set_attenuation(&mut self, attenuation: f64) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogInChannelAttenuationSet self.device_handle, self.index, attenuation)
    }

    pub fn get_attenuation(&self) -> Result<f64, WaveFormsError> {
        get_float!(FDwfAnalogInChannelAttenuationGet self.device_handle, self.index)
    }
}

enum_and_support_bitfield! {
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
