use crate::*;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct WaveformGenerator<'handle> {
    pub(crate) device_handle: c_int,
    pub(crate) phantom: std::marker::PhantomData<&'handle ()>,
}

impl<'handle> WaveformGenerator<'handle> {
    /// Resets all analog output channels
    pub fn reset(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogOutReset self.device_handle, -1)
    }

    pub fn channels(&mut self) -> Result<Vec<Channel<'handle>>, WaveFormsError> {
        let channel_count = get_int!(FDwfAnalogOutCount self.device_handle)?;
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
    pub fn reset(&mut self) -> Result<(), WaveFormsError> {
        call!(FDwfAnalogOutReset self.device_handle, self.index)
    }
}
