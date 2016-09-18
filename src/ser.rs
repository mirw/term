
#[cfg(feature = "std")]
use std;
use core;
use core::{result, fmt};

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::arc::Arc;

#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;

#[cfg(feature = "std")]
use std::string::String;
#[cfg(not(feature = "std"))]
use collections::string::String;

use super::Record;

#[derive(Debug)]
#[cfg(feature = "std")]
/// Serialization Error
pub enum Error {
    /// `io::Error`
    Io(std::io::Error),
    /// Other error
    Other
}

#[derive(Debug)]
#[cfg(not(feature = "std"))]
/// Serialization Error
pub enum Error {
    /// Other error
    Other
}

/// Serialization `Result`
pub type Result = result::Result<(), Error>;

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_ : core::fmt::Error) -> Error {
        Error::Other
    }
}
/*
#[cfg(feature = "std")]
impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        match self {
            Error::Io(e) => e,
            Error::Other => std::io::Error::new(std::io::ErrorKind::Other, "other error"),
        }
    }
}*/

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(e : Error) -> std::io::Error {
        match e {
            Error::Io(e) => e,
            Error::Other => std::io::Error::new(std::io::ErrorKind::Other, "other error"),
        }
    }
}

/// Value that can be serialized
///
/// Loggers require values in key-value pairs to
/// implement this trait.
///
pub trait Serialize {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error>;
}

/// Value that can be serialized and stored
/// in `Logger` itself.
///
/// As Loggers itself must be thread-safe, they can only
/// store values implementing this trait.
pub trait SyncSerialize: Send + Sync + 'static + Serialize {}


/// Serializer
///
/// Drains using `Format` will internally use
/// types implementing this trait.
pub trait Serializer {
    /// Emit bool
    fn emit_bool(&mut self, key: &str, val: bool) -> Result;
    /// Emit `()`
    fn emit_unit(&mut self, key: &str) -> Result;
    /// Emit `None`
    fn emit_none(&mut self, key: &str) -> Result;
    /// Emit char
    fn emit_char(&mut self, key: &str, val: char) -> Result;
    /// Emit u8
    fn emit_u8(&mut self, key: &str, val: u8) -> Result;
    /// Emit i8
    fn emit_i8(&mut self, key: &str, val: i8) -> Result;
    /// Emit u16
    fn emit_u16(&mut self, key: &str, val: u16) -> Result;
    /// Emit i16
    fn emit_i16(&mut self, key: &str, val: i16) -> Result;
    /// Emit u32
    fn emit_u32(&mut self, key: &str, val: u32) -> Result;
    /// Emit i32
    fn emit_i32(&mut self, key: &str, val: i32) -> Result;
    /// Emit f32
    fn emit_f32(&mut self, key: &str, val: f32) -> Result;
    /// Emit u64
    fn emit_u64(&mut self, key: &str, val: u64) -> Result;
    /// Emit i64
    fn emit_i64(&mut self, key: &str, val: i64) -> Result;
    /// Emit f64
    fn emit_f64(&mut self, key: &str, val: f64) -> Result;
    /// Emit usize
    fn emit_usize(&mut self, key: &str, val: usize) -> Result;
    /// Emit isize
    fn emit_isize(&mut self, key: &str, val: isize) -> Result;
    /// Emit str
    fn emit_str(&mut self, key: &str, val: &str) -> Result;
    /// Emit `fmt::Arguments`
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> Result;
}

macro_rules! impl_serialize_for{
    ($t:ty, $f:ident) => {
        impl Serialize for $t {
            fn serialize(&self, _record : &Record, key : &str, serializer : &mut Serializer)
                         -> result::Result<(), Error> {
                serializer.$f(key, *self)
            }
        }

        impl SyncSerialize for $t where $t : Send+Sync+'static { }
    };
}

impl_serialize_for!(usize, emit_usize);
impl_serialize_for!(isize, emit_isize);
impl_serialize_for!(bool, emit_bool);
impl_serialize_for!(char, emit_char);
impl_serialize_for!(u8, emit_u8);
impl_serialize_for!(i8, emit_i8);
impl_serialize_for!(u16, emit_u16);
impl_serialize_for!(i16, emit_i16);
impl_serialize_for!(u32, emit_u32);
impl_serialize_for!(i32, emit_i32);
impl_serialize_for!(f32, emit_f32);
impl_serialize_for!(u64, emit_u64);
impl_serialize_for!(i64, emit_i64);
impl_serialize_for!(f64, emit_f64);

impl Serialize for () {
    fn serialize(&self, _record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        serializer.emit_unit(key)
    }
}

impl SyncSerialize for () {}

impl Serialize for str {
    fn serialize(&self, _record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        serializer.emit_str(key, self)
    }
}

impl<'a> Serialize for &'a str {
    fn serialize(&self, _record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        serializer.emit_str(key, self)
    }
}

impl<'a> Serialize for fmt::Arguments<'a>{
    fn serialize(&self, _record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        serializer.emit_arguments(key, self)
    }
}

impl SyncSerialize for fmt::Arguments<'static> {}

impl SyncSerialize for &'static str {}

impl Serialize for String {
    fn serialize(&self, _record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        serializer.emit_str(key, self.as_str())
    }
}

impl SyncSerialize for String {}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        match *self {
            Some(ref s) => s.serialize(record, key, serializer),
            None => serializer.emit_none(key),
        }
    }
}

impl<T: Serialize + Send + Sync + 'static> SyncSerialize for Option<T> {}

impl<T> Serialize for Arc<T>
    where T: Serialize
{
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> SyncSerialize for Arc<T> where T: SyncSerialize {}

impl<T> Serialize for Rc<T>
    where T: Serialize
{
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> Serialize for core::num::Wrapping<T>
    where T: Serialize
{
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        self.0.serialize(record, key, serializer)
    }
}

impl<T> SyncSerialize for core::num::Wrapping<T> where T: SyncSerialize {}

impl<S: 'static + Serialize, F> Serialize for F
    where F: 'static + for<'c, 'd> Fn(&'c Record<'d>) -> S
{
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        (*self)(record).serialize(record, key, serializer)
    }
}

impl<S: 'static + Serialize, F> SyncSerialize for F
    where F: 'static + Sync + Send + for<'c, 'd> Fn(&'c Record<'d>) -> S
{
}

/// A newtype for non-return based lazy values
///
/// It's more natural for closures used as lazy values to return `Serialize`
/// implementing type, but sometimes that forces an allocation (eg. Strings)
///
/// In some cases it might make sense for another closure form to be used - one
/// taking a serializer as an argument, which avoids lifetimes / allocation issues.
///
/// Unfortunately, as one `struct` can implement many different closure traits,
/// a newtype has to be used to prevent ambiguity.
///
/// Generally this method should be used only if it avoids a big allocation of
/// `Serialize`-implementing type in performance-critical logging statement.
///
/// TODO: Can `PushLazy` be avoided?
pub struct PushLazy<F>(pub F);

/// A handle to `Serializer` for `PushLazy` closure
pub struct ValueSerializer<'a> {
    record: &'a Record<'a>,
    key: &'a str,
    serializer: &'a mut Serializer,
    done: bool,
}

impl<'a> ValueSerializer<'a> {
    /// Serialize a value
    ///
    /// This consumes `self` to prevent serializing one value multiple times
    pub fn serialize<'b, S: 'b + Serialize>(mut self, s: S) -> result::Result<(), Error> {
        self.done = true;
        s.serialize(self.record, self.key, self.serializer)
    }
}

impl<'a> Drop for ValueSerializer<'a> {
    fn drop(&mut self) {
        if !self.done {
            // unfortunately this gives no change to return serialization errors
            let _ = self.serializer.emit_unit(self.key);
        }
    }
}

impl<F> Serialize for PushLazy<F>
    where F: 'static + for<'c, 'd> Fn(&'c Record<'d>, ValueSerializer<'c>) -> result::Result<(), Error>
{
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer) -> result::Result<(), Error> {
        let ser = ValueSerializer {
            record: record,
            key: key,
            serializer: serializer,
            done: false,
        };
        (self.0)(record, ser)
    }
}

impl<F> SyncSerialize for PushLazy<F>
     where F: 'static + Sync + Send + for<'c, 'd> Fn(&'c Record<'d>, ValueSerializer<'c>)
     -> result::Result<(), Error> {
}
