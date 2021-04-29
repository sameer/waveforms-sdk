use super::bindings::*;

// device enumeration filters
#[repr(i32)]
#[derive(Debug)]
pub enum EnumFilter {
    All = enumfilterAll,
    EExplorer = enumfilterEExplorer,
    Discovery = enumfilterDiscovery,
    Discovery2 = enumfilterDiscovery2,
    DDiscovery = enumfilterDDiscovery,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DeviceType {
    ElectronicsExplorer,
    AnalogDiscovery,
    AnalogDiscovery2,
    DigitalDiscovery,
    AnalogDiscoveryPro,
    Unknown,
}

impl Into<DeviceType> for std::os::raw::c_int {
    fn into(self) -> DeviceType {
        use DeviceType::*;
        match self {
            devidEExplorer => ElectronicsExplorer,
            devidDiscovery => AnalogDiscovery,
            devidDiscovery2 => AnalogDiscovery2,
            devidDDiscovery => DigitalDiscovery,
            devidADP3X50 => AnalogDiscoveryPro,
            _ => Unknown,
        }
    }
}
// device ID
#[repr(i32)]
#[derive(Debug)]
pub enum DeviceId {
    EExplorer = devidEExplorer,
    Discovery = devidDiscovery,
    Discovery2 = devidDiscovery2,
    DDiscovery = devidDDiscovery,
    ADP3X50 = devidADP3X50,
}
// device version
#[repr(i32)]
#[derive(Debug)]
pub enum DeviceVersion {
    // TODO: these two have the same discriminant
    EExplorerCOrDiscoveryB = devverEExplorerC,
    EExplorerE = devverEExplorerE,
    EExplorerF = devverEExplorerF,
    DiscoveryA = devverDiscoveryA,
    DiscoveryC = devverDiscoveryC,
}
// trigger source
#[repr(u8)]
#[derive(Debug)]
pub enum TriggerSource {
    None = trigsrcNone,
    PC = trigsrcPC,
    DetectorAnalogIn = trigsrcDetectorAnalogIn,
    DetectorDigitalIn = trigsrcDetectorDigitalIn,
    AnalogIn = trigsrcAnalogIn,
    DigitalIn = trigsrcDigitalIn,
    DigitalOut = trigsrcDigitalOut,
    AnalogOut1 = trigsrcAnalogOut1,
    AnalogOut2 = trigsrcAnalogOut2,
    AnalogOut3 = trigsrcAnalogOut3,
    AnalogOut4 = trigsrcAnalogOut4,
    External1 = trigsrcExternal1,
    External2 = trigsrcExternal2,
    External3 = trigsrcExternal3,
    External4 = trigsrcExternal4,
    High = trigsrcHigh,
    Low = trigsrcLow,
}
// instrument states:
#[repr(u8)]
#[derive(Debug)]
pub enum InstrumentState {
    StateReady = DwfStateReady,
    StateConfig = DwfStateConfig,
    StatePrefill = DwfStatePrefill,
    StateArmed = DwfStateArmed,
    StateWait = DwfStateWait,
    // TODO: triggered and running are both 3
    StateTriggeredOrRunning = DwfStateTriggered,
    StateDone = DwfStateDone,
}

// acquisition modes:
#[repr(i32)]
#[derive(Debug)]
pub enum AcquisitionMode {
    Single = acqmodeSingle,
    ScanShift = acqmodeScanShift,
    ScanScreen = acqmodeScanScreen,
    Record = acqmodeRecord,
    Overs = acqmodeOvers,
    Single1 = acqmodeSingle1,
}
// analog acquisition filter:
#[repr(i32)]
#[derive(Debug)]
pub enum AcquisitionFilter {
    Decimate = filterDecimate,
    Average = filterAverage,
    MinMax = filterMinMax,
}
// analog in trigger mode:
#[repr(i32)]
#[derive(Debug)]
pub enum TriggerType {
    Edge = trigtypeEdge,
    Pulse = trigtypePulse,
    Transition = trigtypeTransition,
    Window = trigtypeWindow,
}
// trigger slope:
#[repr(i32)]
#[derive(Debug)]
pub enum TriggerSlope {
    Rise = DwfTriggerSlopeRise,
    Fall = DwfTriggerSlopeFall,
    Either = DwfTriggerSlopeEither,
}
// trigger length condition
#[repr(i32)]
#[derive(Debug)]
pub enum TriggerLength {
    Less = triglenLess,
    Timeout = triglenTimeout,
    More = triglenMore,
}
// error codes for the functions:
#[repr(i32)]
#[derive(Debug)]
pub enum ErrorCode {
    //  No error occurred
    NoErc = dwfercNoErc,
    //  API waiting on pending API timed out
    UnknownError = dwfercUnknownError,
    //  API waiting on pending API timed out
    ApiLockTimeout = dwfercApiLockTimeout,
    //  Device already opened
    AlreadyOpened = dwfercAlreadyOpened,
    //  Device not supported
    NotSupported = dwfercNotSupported,
    //  Invalid parameter sent in API call
    InvalidParameter0 = dwfercInvalidParameter0,
    //  Invalid parameter sent in API call
    InvalidParameter1 = dwfercInvalidParameter1,
    //  Invalid parameter sent in API call
    InvalidParameter2 = dwfercInvalidParameter2,
    //  Invalid parameter sent in API call
    InvalidParameter3 = dwfercInvalidParameter3,
    //  Invalid parameter sent in API call
    InvalidParameter4 = dwfercInvalidParameter4,
}
// analog out signal types
#[repr(u8)]
#[derive(Debug)]
pub enum GeneratorFunction {
    Dc = funcDC,
    Sine = funcSine,
    Square = funcSquare,
    Triangle = funcTriangle,
    RampUp = funcRampUp,
    RampDown = funcRampDown,
    Noise = funcNoise,
    Pulse = funcPulse,
    Trapezium = funcTrapezium,
    SinePower = funcSinePower,
    Custom = funcCustom,
    Play = funcPlay,
}
// analog io channel node types
#[repr(u8)]
#[derive(Debug)]
pub enum AnalogIoChannelType {
    Enable = analogioEnable,
    Voltage = analogioVoltage,
    Current = analogioCurrent,
    Power = analogioPower,
    Temperature = analogioTemperature,
    Dmm = analogioDmm,
    Range = analogioRange,
    Measure = analogioMeasure,
    Time = analogioTime,
    Frequency = analogioFrequency,
}

#[repr(i32)]
#[derive(Debug)]
pub enum AnalogOutNode {
    Carrier = AnalogOutNodeCarrier,
    FM = AnalogOutNodeFM,
    AM = AnalogOutNodeAM,
}
#[repr(i32)]
#[derive(Debug)]
pub enum AnalogOutMode {
    Voltage = DwfAnalogOutModeVoltage,
    Current = DwfAnalogOutModeCurrent,
}
#[repr(i32)]
#[derive(Debug)]
pub enum AnalogOutIdle {
    Disable = DwfAnalogOutIdleDisable,
    Offset = DwfAnalogOutIdleOffset,
    Initial = DwfAnalogOutIdleInitial,
}
#[repr(i32)]
#[derive(Debug)]
pub enum DigitalInClockSource {
    Internal = DwfDigitalInClockSourceInternal,
    External = DwfDigitalInClockSourceExternal,
    External2 = DwfDigitalInClockSourceExternal2,
}
#[repr(i32)]
#[derive(Debug)]
pub enum DigitalInSampleMode {
    Simple = DwfDigitalInSampleModeSimple,
    // alternate samples: noise|sample|noise|sample|...
    // where noise is more than 1 transition between 2 samples
    Noise = DwfDigitalInSampleModeNoise,
}
#[repr(i32)]
#[derive(Debug)]
pub enum DigitalOutOutput {
    PushPull = DwfDigitalOutOutputPushPull,
    OpenDrain = DwfDigitalOutOutputOpenDrain,
    OpenSource = DwfDigitalOutOutputOpenSource,
    // for custom and random
    ThreeState = DwfDigitalOutOutputThreeState,
}
#[repr(i32)]
#[derive(Debug)]
pub enum DigitalOutType {
    Pulse = DwfDigitalOutTypePulse,
    Custom = DwfDigitalOutTypeCustom,
    Random = DwfDigitalOutTypeRandom,
    ROM = DwfDigitalOutTypeROM,
    State = DwfDigitalOutTypeState,
    Play = DwfDigitalOutTypePlay,
}
#[repr(i32)]
#[derive(Debug)]
pub enum DigitalOutIdle {
    Init = DwfDigitalOutIdleInit,
    Low = DwfDigitalOutIdleLow,
    High = DwfDigitalOutIdleHigh,
    Zet = DwfDigitalOutIdleZet,
}
#[repr(i32)]
#[derive(Debug)]
pub enum AnalogImpedance {
    // Ohms
    Impedance = DwfAnalogImpedanceImpedance,
    // Radians
    ImpedancePhase = DwfAnalogImpedanceImpedancePhase,
    // Ohms
    Resistance = DwfAnalogImpedanceResistance,
    // Ohms
    Reactance = DwfAnalogImpedanceReactance,
    // Siemen
    Admittance = DwfAnalogImpedanceAdmittance,
    // Radians
    AdmittancePhase = DwfAnalogImpedanceAdmittancePhase,
    // Siemen
    Conductance = DwfAnalogImpedanceConductance,
    // Siemen
    Susceptance = DwfAnalogImpedanceSusceptance,
    // Farad
    SeriesCapactance = DwfAnalogImpedanceSeriesCapactance,
    // Farad
    ParallelCapacitance = DwfAnalogImpedanceParallelCapacitance,
    // Henry
    SeriesInductance = DwfAnalogImpedanceSeriesInductance,
    // Henry
    ParallelInductance = DwfAnalogImpedanceParallelInductance,
    // factor
    Dissipation = DwfAnalogImpedanceDissipation,
    // factor
    Quality = DwfAnalogImpedanceQuality,
}
#[repr(i32)]
#[derive(Debug)]
pub enum Param {
    // 1 keep the USB power enabled even when AUX is connected, Analog Discovery 2
    UsbPower = DwfParamUsbPower,
    // LED brightness 0 ... 100%, Digital Discovery
    LedBrightness = DwfParamLedBrightness,
    // 0 continue, 1 stop, 2 shutdown
    OnClose = DwfParamOnClose,
    // 0 disable / 1 enable audio output, Analog Discovery 1, 2
    AudioOut = DwfParamAudioOut,
    // 0..1000 mA USB power limit, -1 no limit, Analog Discovery 1, 2
    UsbLimit = DwfParamUsbLimit,
    // 0 disable / 1 enable
    AnalogOut = DwfParamAnalogOut,
    // MHz
    Frequency = DwfParamFrequency,
}
