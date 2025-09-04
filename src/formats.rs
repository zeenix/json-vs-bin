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

// JSON format implementation
pub struct Json;

impl Json {
    pub fn encode_big(data: &[BigData]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small(data: &[SmallData]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData> {
        serde_json::from_slice(bytes).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData> {
        serde_json::from_slice(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        serde_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData> {
        serde_json::from_slice(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        serde_json::from_slice(bytes).unwrap()
    }
}

// SIMD-JSON format implementation
pub struct SimdJson;

impl SimdJson {
    pub fn encode_big(data: &[BigData]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small(data: &[SmallData]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big(mut bytes: &[u8]) -> Vec<BigData> {
        simd_json::from_reader(&mut bytes).unwrap()
    }

    pub fn decode_small(mut bytes: &[u8]) -> Vec<SmallData> {
        simd_json::from_reader(&mut bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        simd_json::to_string(&data).unwrap().into_bytes()
    }

    pub fn decode_big_vector(mut bytes: &[u8]) -> Vec<BigVectorData> {
        simd_json::from_reader(&mut bytes).unwrap()
    }

    pub fn decode_small_vector(mut bytes: &[u8]) -> Vec<SmallVectorData> {
        simd_json::from_reader(&mut bytes).unwrap()
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

    pub fn encode_big(&self, data: &[BigData]) -> Vec<u8> {
        to_bytes(self.context, &data.to_vec()).unwrap().to_vec()
    }

    pub fn encode_small(&self, data: &[SmallData]) -> Vec<u8> {
        to_bytes(self.context, &data.to_vec()).unwrap().to_vec()
    }

    pub fn decode_big(&self, bytes: &[u8]) -> Vec<BigData> {
        let encoded = Data::new(bytes, self.context);
        let (data, _): (Vec<BigData>, _) = encoded.deserialize().unwrap();
        data
    }

    pub fn decode_small(&self, bytes: &[u8]) -> Vec<SmallData> {
        let encoded = Data::new(bytes, self.context);
        let (data, _): (Vec<SmallData>, _) = encoded.deserialize().unwrap();
        data
    }

    pub fn encode_big_vector(&self, data: &[BigVectorData]) -> Vec<u8> {
        to_bytes(self.context, &data.to_vec()).unwrap().to_vec()
    }

    pub fn encode_small_vector(&self, data: &[SmallVectorData]) -> Vec<u8> {
        to_bytes(self.context, &data.to_vec()).unwrap().to_vec()
    }

    pub fn decode_big_vector(&self, bytes: &[u8]) -> Vec<BigVectorData> {
        let encoded = Data::new(bytes, self.context);
        let (data, _): (Vec<BigVectorData>, _) = encoded.deserialize().unwrap();
        data
    }

    pub fn decode_small_vector(&self, bytes: &[u8]) -> Vec<SmallVectorData> {
        let encoded = Data::new(bytes, self.context);
        let (data, _): (Vec<SmallVectorData>, _) = encoded.deserialize().unwrap();
        data
    }
}

// BSON format implementation
pub struct Bson;

impl Bson {
    pub fn encode_big(data: &[BigData]) -> Vec<u8> {
        let wrapper = BsonWrapper {
            data: data.to_vec(),
        };
        bson::ser::serialize_to_vec(&wrapper).unwrap()
    }

    pub fn encode_small(data: &[SmallData]) -> Vec<u8> {
        let wrapper = BsonWrapper {
            data: data.to_vec(),
        };
        bson::ser::serialize_to_vec(&wrapper).unwrap()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData> {
        let wrapper: BsonWrapper<BigData> = bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData> {
        let wrapper: BsonWrapper<SmallData> = bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }

    pub fn encode_big_vector(data: &[BigVectorData]) -> Vec<u8> {
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

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData> {
        let wrapper: BsonWrapper<BigVectorData> = bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        let wrapper: BsonWrapper<SmallVectorData> =
            bson::de::deserialize_from_slice(bytes).unwrap();
        wrapper.data
    }
}

// CBOR format implementation
pub struct Cbor;

impl Cbor {
    pub fn encode_big(data: &[BigData]) -> Vec<u8> {
        let mut encoded = Vec::new();
        ciborium::into_writer(&data, &mut encoded).unwrap();
        encoded
    }

    pub fn encode_small(data: &[SmallData]) -> Vec<u8> {
        let mut encoded = Vec::new();
        ciborium::into_writer(&data, &mut encoded).unwrap();
        encoded
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData> {
        ciborium::from_reader(&bytes[..]).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData> {
        ciborium::from_reader(&bytes[..]).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData]) -> Vec<u8> {
        let mut encoded = Vec::new();
        ciborium::into_writer(&data, &mut encoded).unwrap();
        encoded
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        let mut encoded = Vec::new();
        ciborium::into_writer(&data, &mut encoded).unwrap();
        encoded
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData> {
        ciborium::from_reader(&bytes[..]).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        ciborium::from_reader(&bytes[..]).unwrap()
    }
}

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

    pub fn encode_big(&self, data: &[BigData]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn encode_small(&self, data: &[SmallData]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn decode_big(&self, bytes: &[u8]) -> Vec<BigData> {
        let (decoded, _): (Vec<BigData>, _) =
            bincode::serde::decode_from_slice(bytes, self.config).unwrap();
        decoded
    }

    pub fn decode_small(&self, bytes: &[u8]) -> Vec<SmallData> {
        let (decoded, _): (Vec<SmallData>, _) =
            bincode::serde::decode_from_slice(bytes, self.config).unwrap();
        decoded
    }

    pub fn encode_big_vector(&self, data: &[BigVectorData]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn encode_small_vector(&self, data: &[SmallVectorData]) -> Vec<u8> {
        bincode::serde::encode_to_vec(&data, self.config).unwrap()
    }

    pub fn decode_big_vector(&self, bytes: &[u8]) -> Vec<BigVectorData> {
        let (decoded, _): (Vec<BigVectorData>, _) =
            bincode::serde::decode_from_slice(bytes, self.config).unwrap();
        decoded
    }

    pub fn decode_small_vector(&self, bytes: &[u8]) -> Vec<SmallVectorData> {
        let (decoded, _): (Vec<SmallVectorData>, _) =
            bincode::serde::decode_from_slice(bytes, self.config).unwrap();
        decoded
    }
}

// Bitcode format implementation
pub struct Bitcode;

impl Bitcode {
    pub fn encode_big(data: &[BigData]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn encode_small(data: &[SmallData]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData> {
        bitcode::deserialize(bytes).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData> {
        bitcode::deserialize(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        bitcode::serialize(&data).unwrap()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData> {
        bitcode::deserialize(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        bitcode::deserialize(bytes).unwrap()
    }
}

// Postcard format implementation
pub struct Postcard;

impl Postcard {
    pub fn encode_big(data: &[BigData]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn encode_small(data: &[SmallData]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn decode_big(bytes: &[u8]) -> Vec<BigData> {
        postcard::from_bytes(bytes).unwrap()
    }

    pub fn decode_small(bytes: &[u8]) -> Vec<SmallData> {
        postcard::from_bytes(bytes).unwrap()
    }

    pub fn encode_big_vector(data: &[BigVectorData]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn encode_small_vector(data: &[SmallVectorData]) -> Vec<u8> {
        postcard::to_allocvec(&data).unwrap()
    }

    pub fn decode_big_vector(bytes: &[u8]) -> Vec<BigVectorData> {
        postcard::from_bytes(bytes).unwrap()
    }

    pub fn decode_small_vector(bytes: &[u8]) -> Vec<SmallVectorData> {
        postcard::from_bytes(bytes).unwrap()
    }
}
