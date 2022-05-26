use libc;
use std::{os::unix::prelude::AsRawFd, ptr::null_mut};
use thiserror::Error;

// TODO possibly do without libc?

pub const REV_PI_DEV_FIRST_RIGHT: usize = 32;
pub const REV_PI_DEV_CNT_MAX: usize = 64;
pub const REV_PI_ERROR_MSG_LEN: usize = 256;
pub const KB_PI_LEN: usize = 4096;

#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct SDeviceInfo {
    pub i8uAddress: u8,
    pub i32uSerialNumber: u32,
    pub i16uModuleType: u16,
    pub i16uHW_Revision: u16,
    pub i16uSW_Major: u16,
    pub i16uSW_Minor: u16,
    pub i32uSVN_Revision: u32,
    pub i16uInputLength: u16,
    pub i16uOutputLength: u16,
    pub i16uConfigLength: u16,
    pub i16uBaseOffset: u16,
    pub i16uInputOffset: u16,
    pub i16uOutputOffset: u16,
    pub i16uConfigOffset: u16,
    pub i16uFirstEntry: u16,
    pub i8uModuleState: u8,
    pub i8uActive: u8,
    pub i8uReserve: [u8; 30],
}

// #[derive(Debug)]
// #[repr(u8)]
// pub enum EntryInfoType {
//     Input = 1,
//     Output,
//     Memory,
//     Config,
// }

// #[allow(non_snake_case)]
// #[derive(Debug)]
// #[repr(C)]
// pub struct SEntryInfo {
//     i8uAddress: u8,
//     i8uType: EntryInfoType,
//     i16uIndex: u16,
//     i16uBitLength: u16,
//     i8uBitPos: u8,
//     i16uOffset: u16,
//     i32uDefault: u32,
//     strVarName: [u8; 32],
// }

#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct SPIValue {
    pub i16uAddress: u16,
    pub i8uBit: u8,
    pub i8uValue: u8,
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct SPIVariable {
    pub strVarName: [u8; 32],
    pub i16uAddress: u16,
    pub i8uBit: u8,
    pub i16uLength: u16,
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct SDIOResetCounter {
    pub i8uAddress: u8,
    pub i16uBitfield: u16,
}

// #[allow(non_snake_case)]
// #[derive(Debug, Default)]
// #[repr(C)]
// pub struct SConfigData {
//     bLeft: u8,
//     i16uLen: u16,
//     acData: [u8; 256]
// }

// from piControl.h
#[repr(u64)]
enum KBRequests {
    // reset the piControl driver including the config file
    Reset = 0x4b0c,
    // get the device info of all detected devices
    GetDeviceInfoList,
    // get the device info of one device
    GetDeviceInfo,
    // get the value of one bit in the process image
    GetValue,
    // set the value of one bit in the process image
    SetValue,
    // find a varible defined in piCtory
    FindVariable,
    // copy the exported outputs from a application process image to the real process image
    SetExportedOutputs,
    // try to update the firmware of connected devices
    UpdateDeviceFirmware,
    // set a counter or endocder to 0
    DIOResetCounter,
    // copy the last error message
    GetLastMessage,
    // stop/start IO communication, can be used for I/O simulation
    StopIO,
    // for download of configuration to Master Gateway: stop IO communication completely
    //ConfigStop,
    // for download of configuration to Master Gateway: download config data
    //ConfigSend,
    // for download of configuration to Master Gateway: restart IO communication
    //ConfigStart,
    // activate a watchdog for this handle. If write is not called for a given period all outputs are set to 0
    SetOutputWatchdog,
    // set the f_pos, the unsigned int * is used to interpret the pos value
    //SetPos,
    //AIOCalibrate,
    // wait for an event. This call is normally blocking
    WaitForEvent = 0x4b32,
}

pub type RawRawResult = Result<u32, i32>;

unsafe fn ioctl<F: AsRawFd, T>(fd: F, request: KBRequests, argp: T) -> RawRawResult {
    let res = libc::ioctl(fd.as_raw_fd(), request as libc::c_ulong, argp);
    if res <= -1 {
        Err(*libc::__errno_location())
    } else {
        Ok(res as u32)
    }
}

pub unsafe fn reset<F: AsRawFd>(fd: F) -> RawRawResult {
    ioctl(fd, KBRequests::Reset, 0u64)
}

pub unsafe fn get_device_info_list<F: AsRawFd>(fd: F, devs: *mut SDeviceInfo) -> RawRawResult {
    ioctl(fd, KBRequests::GetDeviceInfoList, devs)
}

// TODO by module type of this? see manuam
pub unsafe fn get_device_info<F: AsRawFd>(fd: F, dev: *mut SDeviceInfo) -> RawRawResult {
    ioctl(fd, KBRequests::GetDeviceInfo, dev)
}

// In theory this could be safe since the piControl module checks whether the
// index is inside the bounds, but nevertheless, we could read at any random
// point, interpreting the value in a certain way, which also makes this sorta
// unsafe
pub unsafe fn get_value<F: AsRawFd>(fd: F, val: *mut SPIValue) -> RawRawResult {
    ioctl(fd, KBRequests::GetValue, val)
}

pub unsafe fn set_value<F: AsRawFd>(fd: F, val: *mut SPIValue) -> RawRawResult {
    ioctl(fd, KBRequests::SetValue, val)
}

pub unsafe fn find_variable<F: AsRawFd>(fd: F, var: *mut SPIVariable) -> RawRawResult {
    ioctl(fd, KBRequests::FindVariable, var)
}

// image.len() must be the same as processimage length
pub unsafe fn set_exported_outputs<F: AsRawFd>(fd: F, image: *mut u8) -> RawRawResult {
    ioctl(fd, KBRequests::SetExportedOutputs, image)
}

pub unsafe fn update_device_firmware<F: AsRawFd>(fd: F, module: u32) -> RawRawResult {
    ioctl(fd, KBRequests::UpdateDeviceFirmware, module)
}

// dio_address must be valid
pub unsafe fn dio_reset_counter<F: AsRawFd>(fd: F, ctr: *mut SDIOResetCounter) -> RawRawResult {
    ioctl(fd, KBRequests::DIOResetCounter, ctr)
}

pub unsafe fn get_last_message<F: AsRawFd>(fd: F, msg: *mut i8) -> RawRawResult {
    ioctl(fd, KBRequests::GetLastMessage, msg)
}

pub unsafe fn stop_io<F: AsRawFd>(fd: F, stop: *mut i32) -> RawRawResult {
    ioctl(fd, KBRequests::StopIO, stop)
}

pub unsafe fn set_output_watchdog<F: AsRawFd>(fd: F, millis: *mut u32) -> RawRawResult {
    ioctl(fd, KBRequests::SetOutputWatchdog, millis)
}

pub unsafe fn wait_for_event<F: AsRawFd>(fd: F, event: *mut i32) -> RawRawResult {
    ioctl(fd, KBRequests::WaitForEvent, event)
}