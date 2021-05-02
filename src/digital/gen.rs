use crate::*;
use std::os::raw::c_int;
use uom::si::f64::Frequency;
use uom::si::f64::Time;
use uom::si::frequency::hertz;
use uom::si::time::second;

#[derive(Debug)]
pub struct PatternGenerator<'handle> {
    pub(crate) device_handle: c_int,
    pub(crate) phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> PatternGenerator<'handle> {
    pub fn reset(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutReset self.device_handle)
    }

    pub fn start(&mut self) -> Result<(), WaveFormsError> {
        set_true!(FDwfDigitalOutConfigure self.device_handle)
    }

    pub fn stop(&mut self) -> Result<(), WaveFormsError> {
        set_false!(FDwfDigitalOutConfigure self.device_handle)
    }

    pub fn state(&self) -> Result<InstrumentState, WaveFormsError> {
        Ok(InstrumentState::from(
            get_int!(FDwfDigitalOutStatus self.device_handle)?,
        ))
    }

    pub fn supported_run_time_max(&self) -> Result<Time, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfDigitalOutRunInfo self.device_handle, &mut min, &mut max)?;
        Ok(Time::new::<second>(max))
    }

    pub fn supported_run_time_min(&self) -> Result<Time, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfDigitalOutRunInfo self.device_handle, &mut min, &mut max)?;
        Ok(Time::new::<second>(min))
    }

    pub fn set_run_time(&mut self, time: Time) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutRunSet self.device_handle, time.get::<second>())
    }

    pub fn get_run_time(&self) -> Result<Time, WaveFormsError> {
        get_float!(FDwfDigitalOutRunGet self.device_handle).map(|x| Time::new::<second>(x))
    }

    pub fn supported_wait_time_max(&self) -> Result<Time, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfDigitalOutWaitInfo self.device_handle, &mut min, &mut max)?;
        Ok(Time::new::<second>(max))
    }

    pub fn supported_wait_time_min(&self) -> Result<Time, WaveFormsError> {
        let mut min = 0.;
        let mut max = 0.;
        call!(FDwfDigitalOutWaitInfo self.device_handle, &mut min, &mut max)?;
        Ok(Time::new::<second>(min))
    }

    pub fn set_wait_time(&mut self, time: Time) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutWaitSet self.device_handle, time.get::<second>())
    }

    pub fn get_wait_time(&self) -> Result<Time, WaveFormsError> {
        get_float!(FDwfDigitalOutWaitGet self.device_handle).map(|x| Time::new::<second>(x))
    }

    pub fn supported_repeat_range(&self) -> Result<RangeInclusive<u32>, WaveFormsError> {
        let mut min = 0;
        let mut max = 0;
        call!(FDwfDigitalOutRepeatInfo self.device_handle, &mut min, &mut max)?;
        Ok(min..=max)
    }

    pub fn set_repeat(&mut self, size: u32) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutRepeatSet self.device_handle, size)
    }

    pub fn get_repeat(&self) -> Result<u32, WaveFormsError> {
        Ok(get_int!(FDwfDigitalOutRepeatGet self.device_handle)?)
    }

    /// On-device clock source frequency
    pub fn internal_clock_frequency(&self) -> Result<Frequency, WaveFormsError> {
        get_float!(FDwfDigitalOutInternalClockInfo self.device_handle)
            .map(|x| Frequency::new::<hertz>(x))
    }

    pub fn channels(&mut self) -> Result<Vec<Channel>, WaveFormsError> {
        get_int!(FDwfDigitalOutCount self.device_handle).map(|channel_count| {
            (0..channel_count)
                .map(|channel_index| Channel {
                    device_handle: self.device_handle,
                    index: channel_index,
                    phantom: std::marker::PhantomData,
                })
                .collect::<Vec<_>>()
        })
    }

    /// Set the playback frequency. i.e. 32kHz, 44.1kHz, 48kHz
    pub fn set_play_rate(&mut self, frequency: Frequency) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutPlayRateSet self.device_handle, frequency.get::<hertz>())
    }

    /// A data array of samples for playback.
    /// Bitrate can be 1, 2, 4, 8 or 16.
    ///
    /// The sample count is equal to `data.len() * 8 / bitrate`.
    /// If the bitrate is 16, sample count should be even.
    pub fn set_play_data(&mut self, data: &[u8], bitrate: Bitrate) -> Result<(), WaveFormsError> {
        let sample_count = if bitrate == Bitrate::Sixteen {
            data.len() as c_uint / 2
        } else {
            data.len() as c_uint * (8u32 / Into::<u32>::into(bitrate))
        };
        call!(FDwfDigitalOutPlayDataSet self.device_handle, data.as_ptr() as *mut c_uchar, bitrate.into(), sample_count)
    }
}

enum_only! {
    Bitrate c_uint {
        One => 1,
        Two => 2,
        Four => 4,
        Eight => 8,
        Sixteen => 16
    }
}

pub struct Channel<'handle> {
    device_handle: c_int,
    index: c_int,
    phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> Channel<'handle> {
    pub fn enable(&mut self) -> Result<(), WaveFormsError> {
        set_true!(FDwfDigitalOutEnableSet self.device_handle, self.index)
    }

    pub fn disable(&mut self) -> Result<(), WaveFormsError> {
        set_false!(FDwfDigitalOutEnableSet self.device_handle, self.index)
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutOutputSet self.device_handle, self.index, mode.into())
    }

    pub fn get_mode(&self) -> Result<Mode, WaveFormsError> {
        Ok(Mode::from(
            get_int!(FDwfDigitalOutOutputGet self.device_handle, self.index)?,
        ))
    }

    pub fn supported_modes(&self) -> Result<SupportedModes, WaveFormsError> {
        get_int!(FDwfDigitalOutOutputInfo self.device_handle, self.index).map(SupportedModes::from)
    }

    pub fn set_type(&mut self, ty: Type) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutTypeSet self.device_handle, self.index, ty.into())
    }

    pub fn get_type(&self) -> Result<Type, WaveFormsError> {
        Ok(Type::from(
            get_int!(FDwfDigitalOutTypeGet self.device_handle, self.index)?,
        ))
    }

    pub fn supported_types(&self) -> Result<SupportedTypes, WaveFormsError> {
        get_int!(FDwfDigitalOutTypeInfo self.device_handle, self.index).map(SupportedTypes::from)
    }

    pub fn set_idle(&mut self, idle: Idle) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutIdleSet self.device_handle, self.index, idle.into())
    }

    pub fn get_idle(&self) -> Result<Idle, WaveFormsError> {
        Ok(Idle::from(
            get_int!(FDwfDigitalOutIdleGet self.device_handle, self.index)?,
        ))
    }

    pub fn supported_idles(&self) -> Result<SupportedIdles, WaveFormsError> {
        get_int!(FDwfDigitalOutIdleInfo self.device_handle, self.index).map(SupportedIdles::from)
    }

    pub fn supported_divider_range(&self) -> Result<RangeInclusive<u32>, WaveFormsError> {
        let mut min = 0;
        let mut max = 0;
        call!(FDwfDigitalOutDividerInfo self.device_handle, self.index, &mut min, &mut max)?;
        Ok(min..=max)
    }

    pub fn set_initial_divider(&mut self, div: u32) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutDividerInitSet self.device_handle, self.index, div)
    }

    pub fn get_initial_divider(&self) -> Result<u32, WaveFormsError> {
        get_int!(FDwfDigitalOutDividerInitGet self.device_handle, self.index)
    }

    pub fn set_divider(&mut self, div: u32) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutDividerSet self.device_handle, self.index, div)
    }

    pub fn get_divider(&self) -> Result<u32, WaveFormsError> {
        Ok(get_int!(FDwfDigitalOutDividerGet self.device_handle, self.index)?)
    }

    pub fn supported_counter_range(&self) -> Result<RangeInclusive<u32>, WaveFormsError> {
        let mut min = 0;
        let mut max = 0;
        call!(FDwfDigitalOutCounterInfo self.device_handle, self.index, &mut min, &mut max)?;
        Ok(min..=max)
    }

    pub fn set_initial_counter(
        &mut self,
        counter_high: u32,
        div: u32,
    ) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutCounterInitSet self.device_handle, self.index, counter_high as i32, div)
    }

    pub fn get_initial_counter(&self) -> Result<(u32, u32), WaveFormsError> {
        let mut counter_high = 0;
        let mut div = 0;
        call!(FDwfDigitalOutCounterInitGet self.device_handle, self.index, &mut counter_high, &mut div)?;
        Ok((counter_high as u32, div))
    }

    pub fn set_counter(&mut self, low: u32, high: u32) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutCounterSet self.device_handle, self.index, low, high)
    }

    pub fn get_counter(&self) -> Result<(u32, u32), WaveFormsError> {
        let mut min = 0;
        let mut max = 0;
        call!(FDwfDigitalOutCounterGet self.device_handle, self.index, &mut min, &mut max)?;
        Ok((min, max))
    }

    pub fn custom_data_max_length(&self) -> Result<usize, WaveFormsError> {
        use std::convert::TryFrom;
        get_int!(FDwfDigitalOutDataInfo self.device_handle, self.index)
            .map(|len| usize::try_from(len).unwrap_or(usize::MAX))
    }

    /// Also sets the counter initial, low and high value, according the number of bits.
    /// The data bits are sent out in LSB first order.
    /// For TS output, the count of bits is the total number of output value (I/O) and output enable (OE) bits, which should be an even number.
    pub fn set_custom_data(&mut self, bits: &[u8]) -> Result<(), WaveFormsError> {
        call!(FDwfDigitalOutDataSet self.device_handle, self.index, bits.as_ptr() as *mut c_void, bits.len() as c_uint)
    }
}

enum_and_support_bitfield! {
    Mode c_int {
        /// The device will supply an active high or low signal.
        /// https://en.wikipedia.org/wiki/Push%E2%80%93pull_output#Digital_circuits
        PushPull => DwfDigitalOutOutputPushPull,
        /// Good for draining current.
        /// Requires an external pull.
        /// https://en.wikipedia.org/wiki/Open_collector#MOSFET
        OpenDrain => DwfDigitalOutOutputOpenDrain,
        /// Good for sourcing current.
        /// Requires an external pull.
        OpenSource => DwfDigitalOutOutputOpenSource,
        /// Pin can supply a high/low signal or be in high impedance.
        /// Often expects an external pull-up or pull-down resistor.
        /// https://en.wikipedia.org/wiki/Three-state_logic
        /// Used with custom or random.
        Tristate => DwfDigitalOutOutputThreeState
    }
}

enum_and_support_bitfield! {
    Type c_int {
        Pulse => DwfDigitalOutTypePulse,
        Custom => DwfDigitalOutTypeCustom,
        Random => DwfDigitalOutTypeRandom,
        ROM => DwfDigitalOutTypeROM,
        State => DwfDigitalOutTypeState,
        Play => DwfDigitalOutTypePlay
    }
}

enum_and_support_bitfield! {
    Idle c_int {
        /// Keeps outputting the last value
        Init => DwfDigitalOutIdleInit,
        Low => DwfDigitalOutIdleLow,
        High => DwfDigitalOutIdleHigh,
        /// Tristate mode high impedance
        Tristate => DwfDigitalOutIdleZet
    }
}
