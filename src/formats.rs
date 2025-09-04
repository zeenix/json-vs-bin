use crate::data::{BigData, SmallData};
use serde::{Deserialize, Serialize};
use zvariant::{
    serialized::{Context, Data},
    to_bytes_for_signature, Endian, Type,
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
        let signature = <Vec<BigData> as Type>::SIGNATURE;
        to_bytes_for_signature(self.context, signature, &data.to_vec())
            .unwrap()
            .to_vec()
    }

    pub fn encode_small(&self, data: &[SmallData]) -> Vec<u8> {
        let signature = <Vec<SmallData> as Type>::SIGNATURE;
        to_bytes_for_signature(self.context, signature, &data.to_vec())
            .unwrap()
            .to_vec()
    }

    pub fn decode_big(&self, bytes: &[u8]) -> Vec<BigData> {
        let signature = <Vec<BigData> as Type>::SIGNATURE;
        let encoded = Data::new(bytes, self.context);
        let (data, _): (Vec<BigData>, _) = encoded.deserialize_for_signature(signature).unwrap();
        data
    }

    pub fn decode_small(&self, bytes: &[u8]) -> Vec<SmallData> {
        let signature = <Vec<SmallData> as Type>::SIGNATURE;
        let encoded = Data::new(bytes, self.context);
        let (data, _): (Vec<SmallData>, _) = encoded.deserialize_for_signature(signature).unwrap();
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
}
