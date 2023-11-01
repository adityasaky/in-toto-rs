//! Structures and functions to aid in various in-toto data interchange formats.

pub(crate) mod cjson;
pub use cjson::{Json, JsonPretty};

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fmt::Debug;
use std::io::{Read, Write};

use crate::Result;

/// The format used for data interchange, serialization, and deserialization.
pub trait DataInterchange: Debug + PartialEq + Clone {
    /// The type of data that is contained in the `signed` portion of metadata.
    type RawData: Serialize + DeserializeOwned + Clone + PartialEq;

    /// The data interchange's extension.
    fn extension() -> &'static str;

    /// A function that canonicalizes data to allow for deterministic signatures.
    fn canonicalize(raw_data: &Self::RawData) -> Result<Vec<u8>>;

    /// Deserialize from `RawData`.
    fn deserialize<T>(raw_data: &Self::RawData) -> Result<T>
    where
        T: DeserializeOwned;

    /// Serialize into `RawData`.
    fn serialize<T>(data: &T) -> Result<Self::RawData>
    where
        T: Serialize;

    /// Write a struct to a stream.
    #[allow(clippy::wrong_self_convention)]
    fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
    where
        W: Write,
        T: Serialize + Sized;

    /// Read a struct from a stream.
    fn from_reader<R, T>(rdr: R) -> Result<T>
    where
        R: Read,
        T: DeserializeOwned;

    /// Read a struct from a stream.
    fn from_slice<T>(slice: &[u8]) -> Result<T>
    where
        T: DeserializeOwned;
}
