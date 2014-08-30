#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#![allow(dead_code)]

/* automatically generated by rust-bindgen */

pub type PaError = ::libc::c_int;
pub type Enum_PaErrorCode = ::libc::c_int;
pub static paNoError: ::libc::c_int = 0;
pub static paNotInitialized: ::libc::c_int = -10000;
pub static paUnanticipatedHostError: ::libc::c_int = -9999;
pub static paInvalidChannelCount: ::libc::c_int = -9998;
pub static paInvalidSampleRate: ::libc::c_int = -9997;
pub static paInvalidDevice: ::libc::c_int = -9996;
pub static paInvalidFlag: ::libc::c_int = -9995;
pub static paSampleFormatNotSupported: ::libc::c_int = -9994;
pub static paBadIODeviceCombination: ::libc::c_int = -9993;
pub static paInsufficientMemory: ::libc::c_int = -9992;
pub static paBufferTooBig: ::libc::c_int = -9991;
pub static paBufferTooSmall: ::libc::c_int = -9990;
pub static paNullCallback: ::libc::c_int = -9989;
pub static paBadStreamPtr: ::libc::c_int = -9988;
pub static paTimedOut: ::libc::c_int = -9987;
pub static paInternalError: ::libc::c_int = -9986;
pub static paDeviceUnavailable: ::libc::c_int = -9985;
pub static paIncompatibleHostApiSpecificStreamInfo: ::libc::c_int = -9984;
pub static paStreamIsStopped: ::libc::c_int = -9983;
pub static paStreamIsNotStopped: ::libc::c_int = -9982;
pub static paInputOverflowed: ::libc::c_int = -9981;
pub static paOutputUnderflowed: ::libc::c_int = -9980;
pub static paHostApiNotFound: ::libc::c_int = -9979;
pub static paInvalidHostApi: ::libc::c_int = -9978;
pub static paCanNotReadFromACallbackStream: ::libc::c_int = -9977;
pub static paCanNotWriteToACallbackStream: ::libc::c_int = -9976;
pub static paCanNotReadFromAnOutputOnlyStream: ::libc::c_int = -9975;
pub static paCanNotWriteToAnInputOnlyStream: ::libc::c_int = -9974;
pub static paIncompatibleStreamHostApi: ::libc::c_int = -9973;
pub static paBadBufferPtr: ::libc::c_int = -9972;
pub type PaErrorCode = Enum_PaErrorCode;
pub type PaDeviceIndex = ::libc::c_int;
pub type PaHostApiIndex = ::libc::c_int;
pub type Enum_PaHostApiTypeId = ::libc::c_uint;
pub static paInDevelopment: ::libc::c_uint = 0;
pub static paDirectSound: ::libc::c_uint = 1;
pub static paMME: ::libc::c_uint = 2;
pub static paASIO: ::libc::c_uint = 3;
pub static paSoundManager: ::libc::c_uint = 4;
pub static paCoreAudio: ::libc::c_uint = 5;
pub static paOSS: ::libc::c_uint = 7;
pub static paALSA: ::libc::c_uint = 8;
pub static paAL: ::libc::c_uint = 9;
pub static paBeOS: ::libc::c_uint = 10;
pub static paWDMKS: ::libc::c_uint = 11;
pub static paJACK: ::libc::c_uint = 12;
pub static paWASAPI: ::libc::c_uint = 13;
pub static paAudioScienceHPI: ::libc::c_uint = 14;
pub type PaHostApiTypeId = Enum_PaHostApiTypeId;
#[repr(C)]
pub struct Struct_PaHostApiInfo {
    pub structVersion: ::libc::c_int,
    pub _type: PaHostApiTypeId,
    pub name: *const ::libc::c_char,
    pub deviceCount: ::libc::c_int,
    pub defaultInputDevice: PaDeviceIndex,
    pub defaultOutputDevice: PaDeviceIndex,
}
pub type PaHostApiInfo = Struct_PaHostApiInfo;
#[repr(C)]
pub struct Struct_PaHostErrorInfo {
    pub hostApiType: PaHostApiTypeId,
    pub errorCode: ::libc::c_long,
    pub errorText: *const ::libc::c_char,
}
pub type PaHostErrorInfo = Struct_PaHostErrorInfo;
pub type PaTime = ::libc::c_double;
pub type PaSampleFormat = ::libc::c_ulong;
#[repr(C)]
pub struct Struct_PaDeviceInfo {
    pub structVersion: ::libc::c_int,
    pub name: *const ::libc::c_char,
    pub hostApi: PaHostApiIndex,
    pub maxInputChannels: ::libc::c_int,
    pub maxOutputChannels: ::libc::c_int,
    pub defaultLowInputLatency: PaTime,
    pub defaultLowOutputLatency: PaTime,
    pub defaultHighInputLatency: PaTime,
    pub defaultHighOutputLatency: PaTime,
    pub defaultSampleRate: ::libc::c_double,
}
pub type PaDeviceInfo = Struct_PaDeviceInfo;
#[repr(C)]
pub struct Struct_PaStreamParameters {
    pub device: PaDeviceIndex,
    pub channelCount: ::libc::c_int,
    pub sampleFormat: PaSampleFormat,
    pub suggestedLatency: PaTime,
    pub hostApiSpecificStreamInfo: *mut ::libc::c_void,
}
pub type PaStreamParameters = Struct_PaStreamParameters;
pub type PaStream = ::libc::c_void;
pub type PaStreamFlags = ::libc::c_ulong;
#[repr(C)]
pub struct Struct_PaStreamCallbackTimeInfo {
    pub inputBufferAdcTime: PaTime,
    pub currentTime: PaTime,
    pub outputBufferDacTime: PaTime,
}
pub type PaStreamCallbackTimeInfo = Struct_PaStreamCallbackTimeInfo;
pub type PaStreamCallbackFlags = ::libc::c_ulong;
pub type Enum_PaStreamCallbackResult = ::libc::c_uint;
pub static paContinue: ::libc::c_uint = 0;
pub static paComplete: ::libc::c_uint = 1;
pub static paAbort: ::libc::c_uint = 2;
pub type PaStreamCallbackResult = Enum_PaStreamCallbackResult;
pub type PaStreamCallback = ::libc::c_void;
pub type PaStreamFinishedCallback = ::libc::c_void;
#[repr(C)]
pub struct Struct_PaStreamInfo {
    pub structVersion: ::libc::c_int,
    pub inputLatency: PaTime,
    pub outputLatency: PaTime,
    pub sampleRate: ::libc::c_double,
}
pub type PaStreamInfo = Struct_PaStreamInfo;
#[link(name = "portaudio")]
extern "C" {
    pub fn Pa_GetVersion() -> ::libc::c_int;
    pub fn Pa_GetVersionText() -> *const ::libc::c_char;
    pub fn Pa_GetErrorText(errorCode: PaError) -> *const ::libc::c_char;
    pub fn Pa_Initialize() -> PaError;
    pub fn Pa_Terminate() -> PaError;
    pub fn Pa_GetHostApiCount() -> PaHostApiIndex;
    pub fn Pa_GetDefaultHostApi() -> PaHostApiIndex;
    pub fn Pa_GetHostApiInfo(hostApi: PaHostApiIndex) -> *const PaHostApiInfo;
    pub fn Pa_HostApiTypeIdToHostApiIndex(_type: PaHostApiTypeId) ->
     PaHostApiIndex;
    pub fn Pa_HostApiDeviceIndexToDeviceIndex(hostApi: PaHostApiIndex,
                                              hostApiDeviceIndex:
                                                  ::libc::c_int) ->
     PaDeviceIndex;
    pub fn Pa_GetLastHostErrorInfo() -> *const PaHostErrorInfo;
    pub fn Pa_GetDeviceCount() -> PaDeviceIndex;
    pub fn Pa_GetDefaultInputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDefaultOutputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDeviceInfo(device: PaDeviceIndex) -> *const PaDeviceInfo;
    pub fn Pa_IsFormatSupported(inputParameters: *const PaStreamParameters,
                                outputParameters: *const PaStreamParameters,
                                sampleRate: ::libc::c_double) -> PaError;
    pub fn Pa_OpenStream(stream: *mut *mut PaStream,
                         inputParameters: *const PaStreamParameters,
                         outputParameters: *const PaStreamParameters,
                         sampleRate: ::libc::c_double,
                         framesPerBuffer: ::libc::c_ulong,
                         streamFlags: PaStreamFlags,
                         streamCallback: extern "C" fn(arg1: *const ::libc::c_void,
                                                       arg2: *mut ::libc::c_void,
                                                       arg3: ::libc::c_ulong,
                                                       arg4: *const PaStreamCallbackTimeInfo,
                                                       arg5: PaStreamCallbackFlags,
                                                       arg6: *mut ::libc::c_void)
                                                      -> ::libc::c_int,
                         userData: *mut ::libc::c_void) -> PaError;
    pub fn Pa_OpenDefaultStream(stream: *mut *mut PaStream,
                                numInputChannels: ::libc::c_int,
                                numOutputChannels: ::libc::c_int,
                                sampleFormat: PaSampleFormat,
                                sampleRate: ::libc::c_double,
                                framesPerBuffer: ::libc::c_ulong,
                                streamCallback: extern "C" fn(arg1: *const ::libc::c_void,
                                                              arg2: *mut ::libc::c_void,
                                                              arg3: ::libc::c_ulong,
                                                              arg4: *const PaStreamCallbackTimeInfo,
                                                              arg5: PaStreamCallbackFlags,
                                                              arg6: *mut ::libc::c_void)
                                                             -> ::libc::c_int,
                                userData: *mut ::libc::c_void) -> PaError;
    pub fn Pa_CloseStream(stream: *mut PaStream) -> PaError;
    pub fn Pa_SetStreamFinishedCallback(stream: *mut PaStream,
                                        streamFinishedCallback: extern "C" fn(arg1: *mut ::libc::c_void))
                                       -> PaError;
    pub fn Pa_StartStream(stream: *mut PaStream) -> PaError;
    pub fn Pa_StopStream(stream: *mut PaStream) -> PaError;
    pub fn Pa_AbortStream(stream: *mut PaStream) -> PaError;
    pub fn Pa_IsStreamStopped(stream: *mut PaStream) -> PaError;
    pub fn Pa_IsStreamActive(stream: *mut PaStream) -> PaError;
    pub fn Pa_GetStreamInfo(stream: *mut PaStream) -> *const PaStreamInfo;
    pub fn Pa_GetStreamTime(stream: *mut PaStream) -> PaTime;
    pub fn Pa_GetStreamCpuLoad(stream: *mut PaStream) -> ::libc::c_double;
    pub fn Pa_ReadStream(stream: *mut PaStream, buffer: *mut ::libc::c_void,
                         frames: ::libc::c_ulong) -> PaError;
    pub fn Pa_WriteStream(stream: *mut PaStream,
                          buffer: *const ::libc::c_void,
                          frames: ::libc::c_ulong) -> PaError;
    pub fn Pa_GetStreamReadAvailable(stream: *mut PaStream) -> ::libc::c_long;
    pub fn Pa_GetStreamWriteAvailable(stream: *mut PaStream) ->
     ::libc::c_long;
    pub fn Pa_GetSampleSize(format: PaSampleFormat) -> PaError;
    pub fn Pa_Sleep(msec: ::libc::c_long);
}
