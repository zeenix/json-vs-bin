use crate::data::{BigData, SmallData};
use crate::vector_data::{BigVectorData, SmallVectorData};
use serde::{Deserialize, Serialize};
use zvariant::{
    serialized::{Context, Data},
    to_bytes, Endian,
};

// Helper struct for BSON which can't handle arrays at the top level.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct BsonWrapper<D> {
    pub data: Vec<D>,
}

// BSON format implementation
pub struct Bson;

impl Bson {
    pub fn encode_big(data: &[BigData<'_>]) -> Vec<u8> {
        let wrapper = BsonWrapper {
            data: data.to_vec(),
        };
        bson::ser::serialize_to_vec(&wrapper).unwrap()
    }

    pub fn encode_small(data: &[SmallData<'_>]) -> Vec<u8> {
        let wrapper = BsonWrapper {
            data: data.to_vec(),
        };
        bson::ser::serialize_to_vec(&wrapper).unwrap()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData<'_>> {
        // Use bson::deserialize_from_slice for zero-copy where possible
        let wrapper: BsonWrapper<BigData> = bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData<'_>> {
        let wrapper: BsonWrapper<SmallData> = bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }

    pub fn encode_big_vector(data: &[BigVectorData<'_>]) -> Vec<u8> {
        let wrapper = BsonWrapper {
            data: data.to_vec(),
        };
        bson::ser::serialize_to_vec(&wrapper).unwrap()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        let wrapper = BsonWrapper {
            data: data.to_vec(),
        };
        bson::ser::serialize_to_vec(&wrapper).unwrap()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData<'_>> {
        let wrapper: BsonWrapper<BigVectorData> = bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        let wrapper: BsonWrapper<SmallVectorData> =
            bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }
}

// JSON format implementation
pub struct Json;

impl Json {
    pub fn encode_big(data: &[BigData<'_>]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small(data: &[SmallData<'_>]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData<'_>> {
        // JSON can do zero-copy if strings don't need unescaping
        serde_json::from_slice(bytes).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData<'_>> {
        serde_json::from_slice(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData<'_>]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData<'_>> {
        serde_json::from_slice(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        serde_json::from_slice(bytes).unwrap()
    }
}

// SIMD-JSON format implementation
pub struct SimdJson;

impl SimdJson {
    pub fn encode_big(data: &[BigData<'_>]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small(data: &[SmallData<'_>]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big(bytes: &mut [u8]) -> Vec<BigData<'_>> {
        // SIMD-JSON requires mutable buffer
        simd_json::from_slice(bytes).unwrap()
    }

    pub fn decode_small(bytes: &mut [u8]) -> Vec<SmallData<'_>> {
        simd_json::from_slice(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData<'_>]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big_vector(bytes: &mut [u8]) -> Vec<BigVectorData<'_>> {
        simd_json::from_slice(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &mut [u8]) -> Vec<SmallVectorData> {
        simd_json::from_slice(bytes).unwrap()
    }
}

// D-Bus format implementation
pub struct DBus {
    context: Context,
}

impl DBus {
    pub fn new() -> Self {
        Self {
            context: Context::new_dbus(Endian::Little, 0),
        }
    }

    pub fn encode_big(&self, data: &[BigData<'_>]) -> Data<'static, 'static> {
        to_bytes(self.context, &data.to_vec()).unwrap()
    }

    pub fn encode_small(&self, data: &[SmallData<'_>]) -> Data<'static, 'static> {
        to_bytes(self.context, &data.to_vec()).unwrap()
    }

    pub fn decode_big<'a>(&self, encoded: &'a Data<'a, 'static>) -> Vec<BigData<'a>> {
        // Use Data's deserialize method for zero-copy
        let (decoded, _): (Vec<BigData>, _) = encoded.deserialize().unwrap();
        decoded
    }

    pub fn decode_small<'a>(&self, encoded: &'a Data<'a, 'static>) -> Vec<SmallData<'a>> {
        // Use Data's deserialize method for zero-copy
        let (decoded, _): (Vec<SmallData>, _) = encoded.deserialize().unwrap();
        decoded
    }

    pub fn encode_big_vector(&self, data: &[BigVectorData<'_>]) -> Data<'static, 'static> {
        to_bytes(self.context, &data.to_vec()).unwrap()
    }

    pub fn encode_small_vector(&self, data: &[SmallVectorData]) -> Data<'static, 'static> {
        to_bytes(self.context, &data.to_vec()).unwrap()
    }

    pub fn decode_big_vector<'a>(&self, encoded: &'a Data<'a, 'static>) -> Vec<BigVectorData<'a>> {
        // Use Data's deserialize method for zero-copy
        let (decoded, _): (Vec<BigVectorData>, _) = encoded.deserialize().unwrap();
        decoded
    }

    pub fn decode_small_vector(&self, encoded: &Data<'_, 'static>) -> Vec<SmallVectorData> {
        // Use Data's deserialize method for zero-copy
        let (decoded, _): (Vec<SmallVectorData>, _) = encoded.deserialize().unwrap();
        decoded
    }
}

// CBOR removed - ciborium has serde trait limitations preventing zero-copy with &str fields

// Bincode format implementation
pub struct Bincode {
    config: bincode::config::Configuration,
}

impl Bincode {
    pub fn new() -> Self {
        Self {
            config: bincode::config::standard(),
        }
    }

    pub fn encode_big(&self, data: &[BigData<'_>]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn encode_small(&self, data: &[SmallData<'_>]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn decode_big<'a>(&self, bytes: &'a [u8]) -> Vec<BigData<'a>> {
        // Use bincode's borrow API for zero-copy
        let (decoded, _) = bincode::serde::borrow_decode_from_slice(bytes, self.config).unwrap();
        decoded
    }

    pub fn decode_small<'a>(&self, bytes: &'a [u8]) -> Vec<SmallData<'a>> {
        let (decoded, _) = bincode::serde::borrow_decode_from_slice(bytes, self.config).unwrap();
        decoded
    }

    pub fn encode_big_vector(&self, data: &[BigVectorData<'_>]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn encode_small_vector(&self, data: &[SmallVectorData]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn decode_big_vector<'a>(&self, bytes: &'a [u8]) -> Vec<BigVectorData<'a>> {
        let (decoded, _) = bincode::serde::borrow_decode_from_slice(bytes, self.config).unwrap();
        decoded
    }

    pub fn decode_small_vector(&self, bytes: &[u8]) -> Vec<SmallVectorData> {
        let (decoded, _) = bincode::serde::borrow_decode_from_slice(bytes, self.config).unwrap();
        decoded
    }
}

// Bitcode format implementation
pub struct Bitcode;

impl Bitcode {
    pub fn encode_big(data: &[BigData<'_>]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn encode_small(data: &[SmallData<'_>]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData<'_>> {
        // Bitcode supports zero-copy deserialization
        bitcode::deserialize(bytes).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData<'_>> {
        bitcode::deserialize(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData<'_>]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData<'_>> {
        bitcode::deserialize(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        bitcode::deserialize(bytes).unwrap()
    }
}

// Postcard format implementation
pub struct Postcard;

impl Postcard {
    pub fn encode_big(data: &[BigData<'_>]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn encode_small(data: &[SmallData<'_>]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData<'_>> {
        // Postcard supports zero-copy deserialization
        postcard::from_bytes(bytes).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData<'_>> {
        postcard::from_bytes(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData<'_>]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData<'_>> {
        postcard::from_bytes(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        postcard::from_bytes(bytes).unwrap()
    }
}
