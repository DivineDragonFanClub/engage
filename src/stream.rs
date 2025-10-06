use std::io;

use unity::prelude::*;

#[unity::class("App", "Stream")]
pub struct Stream {
    buffer: &'static mut Il2CppArray<u8>,
    position: i32,
    stack: *const u8 ,
}

#[unity::from_offset("App", "Stream", "WriteInt")]
extern "C" fn stream_write_int(this: &mut Stream, data: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Stream", "ReadInt")]
extern "C" fn stream_read_int(this: &mut Stream, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Stream", "WriteBegin")]
extern "C" fn stream_write_begin(this: &mut Stream, version: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Stream", "WriteEnd")]
extern "C" fn stream_write_end(this: &mut Stream, method_info: OptionalMethod);
#[skyline::from_offset(0x2508660)]
fn stream_read_begin(this: &mut Stream, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Stream", "ReadEnd")]
extern "C" fn stream_read_end(this: &mut Stream, warning: bool, method_info: OptionalMethod);

#[skyline::from_offset(0x02505830)]
fn stream_write_i64(this: &mut Stream, data: i64, method_info: OptionalMethod);

#[skyline::from_offset(0x02506de0)]
fn stream_read_i64(this: &mut Stream, method_info: OptionalMethod) -> i64;

#[skyline::from_offset(0x02505060)]
fn stream_write_u8(this: &mut Stream, data: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x02506610)]
fn stream_read_u8(this: &mut Stream, method_info: OptionalMethod) -> u8;

impl Stream {
    pub fn get_position(&self) -> usize {
        self.position as _
    }

    /// /!\ To be deprecated in favor of a std::io::Write implementation in due time.
    /// 
    /// Write a i32 into the stream and returns how many bytes were written.
    /// An error of type WriteZero is returned if the buffer was too small to write the value.
    /// 
    /// While useless at the moment, this result type was chosen to facilitate migrating to proper Rust utilities later.
    pub fn write_int(&mut self, data: i32) -> io::Result<usize> {
        if self.buffer.len() >= self.get_position() + 4 {
            unsafe { stream_write_int(self, data, None) }
            Ok(4)
        } else {
            Err(io::Error::from(io::ErrorKind::WriteZero))
        }
    }
    pub fn write_u8(&mut self, data: u8) -> io::Result<usize> {
        if self.buffer.len() >= self.get_position() + 1 {
            unsafe { stream_write_u8(self, data, None) }
            Ok(1)
        } else { Err(io::Error::from(io::ErrorKind::WriteZero)) }
    }
    pub fn write_i64(&mut self, data: i64) -> io::Result<usize> {
        if self.buffer.len() >= self.get_position() + 8 {
            unsafe { stream_write_i64(self, data, None) }
            Ok(8)
        } else { Err(io::Error::from(io::ErrorKind::WriteZero)) }
    }

    /// /!\ To be deprecated in favor of a std::io::Read implementation in due time.
    /// 
    /// Read a i32 from the stream and returns the value.
    /// An error of type UnexpectedEof is returned if the buffer did not have enough bytes left to read the value.
    /// 
    /// While useless at the moment, this result type was chosen to facilitate migrating to proper Rust utilities later.
    pub fn read_int(&mut self) -> io::Result<i32> {
        if self.buffer.len() >= self.get_position() + 4 {
            Ok(unsafe { stream_read_int(self, None) })
        } else {
            Err(io::Error::from(io::ErrorKind::UnexpectedEof))
        }
    }
    pub fn read_i64(&mut self) -> io::Result<i64> {
        if self.buffer.len() >= self.get_position() + 8 { Ok(unsafe { stream_read_i64(self, None) }) }
        else { Err(io::Error::from(io::ErrorKind::UnexpectedEof)) }
    }
    pub fn read_u8(&mut self) -> io::Result<u8> {
        if self.buffer.len() >= self.get_position() + 8 { Ok(unsafe { stream_read_u8(self, None) }) }
        else { Err(io::Error::from(io::ErrorKind::UnexpectedEof)) }
    }
    pub fn write_begin(&mut self, version: i32){ unsafe { stream_write_begin(self, version, None); } }
    pub fn write_end(&mut self) { unsafe { stream_write_end(self, None); } }
    pub fn read_begin(&mut self) -> i32 { unsafe { stream_read_begin(self, None) } }
    pub fn read_end(&mut self, warning: bool) { unsafe { stream_read_end(self, warning, None)}}
}