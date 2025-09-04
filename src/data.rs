use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter};
use zvariant::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct BigData {
    pub int1_loooooooooooooooooooooooooooooooooooong_name: u64,
    pub int2_loooooooooooooooooooooooooooooooooooooong_name: u8,
    pub bool1_looooooooooooooooooooooong_name: bool,
    pub string1: String,
    pub int3_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    pub string2: String,
    pub map1_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
    pub int4: u8,
    pub string3: String,
    pub int5: u32,
    pub map2_also_loooooooooooooong_name: std::collections::HashMap<String, u32>,
    // Repeat previous fields 4 times more but with different names
    pub int6_loooooooooooooooooooooooooooooooooooong_name: u64,
    pub int7_loooooooooooooooooooooooooooooooooooooong_name: u8,
    pub bool2_looooooooooooooooooooooong_name: bool,
    pub string4: String,
    pub int8_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    pub string5: String,
    pub map3_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
    pub int9: u8,
    pub string6: String,
    pub int10: u32,
    pub map4_also_loooooooooooooong_name: std::collections::HashMap<String, u32>,
    pub int11_loooooooooooooooooooooooooooooooooooong_name: u64,
    pub int12_loooooooooooooooooooooooooooooooooooooong_name: u8,
    pub bool3_looooooooooooooooooooooong_name: bool,
    pub string7: String,
    pub int13_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    pub string8: String,
    pub map5_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
}

impl BigData {
    pub fn new() -> Self {
        let string = iter::repeat('o').take(250).collect::<String>();
        let mut map = HashMap::new();
        for i in 0..100 {
            let mut key = string.clone();
            key.push(unsafe { char::from_u32_unchecked(i as u32 + 64) });
            map.insert(key.clone(), i);
        }
        Self {
            int1_loooooooooooooooooooooooooooooooooooong_name: 42,
            int2_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool1_looooooooooooooooooooooong_name: true,
            string1: "Hello, world!".to_string(),
            int3_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string2: string.clone(),
            map1_loooooooooooooooooooooooong_name: map.clone(),
            int4: 42,
            string3: string.clone(),
            int5: 42,
            map2_also_loooooooooooooong_name: map.clone(),
            int6_loooooooooooooooooooooooooooooooooooong_name: 42,
            int7_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool2_looooooooooooooooooooooong_name: true,
            string4: "Hello, world!".to_string(),
            int8_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string5: string.clone(),
            map3_loooooooooooooooooooooooong_name: map.clone(),
            int9: 42,
            string6: string.clone(),
            int10: 42,
            map4_also_loooooooooooooong_name: map.clone(),
            int11_loooooooooooooooooooooooooooooooooooong_name: 42,
            int12_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool3_looooooooooooooooooooooong_name: true,
            string7: "Hello, world!".to_string(),
            int13_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string8: string.clone(),
            map5_loooooooooooooooooooooooong_name: map,
        }
    }
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct SmallData {
    pub int1_loooooooooooooooooooooooooooooooooooong_name: u64,
    pub int2_loooooooooooooooooooooooooooooooooooooong_name: u8,
    pub bool1_looooooooooooooooooooooong_name: bool,
    pub string1: String,
    pub int3_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    pub string2: String,
    pub map1_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
}

impl SmallData {
    pub fn new() -> Self {
        let string = iter::repeat('o').take(250).collect::<String>();
        let mut map = HashMap::new();
        for i in 0..100 {
            let mut key = string.clone();
            key.push(unsafe { char::from_u32_unchecked(i as u32 + 64) });
            map.insert(key.clone(), i);
        }
        Self {
            int1_loooooooooooooooooooooooooooooooooooong_name: 42,
            int2_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool1_looooooooooooooooooooooong_name: true,
            string1: "Hello, world!".to_string(),
            int3_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string2: string.clone(),
            map1_loooooooooooooooooooooooong_name: map,
        }
    }
}
