use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter};
use zvariant::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct BigData {
    pub user_id: u64,
    pub status_code: u8,
    pub is_active: bool,
    pub name: String,
    pub retry_count: u8,
    pub description: String,
    pub properties: std::collections::HashMap<String, u32>,
    pub priority: u8,
    pub message: String,
    pub sequence_num: u32,
    pub metadata: std::collections::HashMap<String, u32>,
    // Additional fields with realistic names
    pub session_id: u64,
    pub error_code: u8,
    pub is_verified: bool,
    pub title: String,
    pub attempt_count: u8,
    pub content: String,
    pub attributes: std::collections::HashMap<String, u32>,
    pub level: u8,
    pub details: String,
    pub request_id: u32,
    pub headers: std::collections::HashMap<String, u32>,
    pub timestamp: u64,
    pub response_code: u8,
    pub is_complete: bool,
    pub summary: String,
    pub max_retries: u8,
    pub body: String,
    pub tags: std::collections::HashMap<String, u32>,
}

impl BigData {
    pub fn new() -> Self {
        // Create realistic HashMap with shorter keys (e.g., "prop_0", "prop_1", etc.).
        let mut properties = HashMap::new();
        for i in 0..100 {
            properties.insert(format!("prop_{}", i), i);
        }

        // Different HashMap instances with different key patterns to simulate variety.
        let mut metadata = HashMap::new();
        for i in 0..100 {
            metadata.insert(format!("meta_{}", i), i * 2);
        }

        let mut attributes = HashMap::new();
        for i in 0..100 {
            attributes.insert(format!("attr_{}", i), i * 3);
        }

        let mut headers = HashMap::new();
        for i in 0..100 {
            headers.insert(format!("header_{}", i), i * 4);
        }

        let mut tags = HashMap::new();
        for i in 0..100 {
            tags.insert(format!("tag_{}", i), i * 5);
        }

        let long_string = iter::repeat('o').take(250).collect::<String>();

        Self {
            user_id: 42,
            status_code: 200,
            is_active: true,
            name: "John Doe".to_string(),
            retry_count: 3,
            description: long_string.clone(),
            properties,
            priority: 1,
            message: "Processing request".to_string(),
            sequence_num: 12345,
            metadata,
            session_id: 987654321,
            error_code: 0,
            is_verified: true,
            title: "Important Task".to_string(),
            attempt_count: 2,
            content: long_string.clone(),
            attributes,
            level: 5,
            details: long_string.clone(),
            request_id: 67890,
            headers,
            timestamp: 1640000000,
            response_code: 201,
            is_complete: false,
            summary: "Task in progress".to_string(),
            max_retries: 10,
            body: long_string,
            tags,
        }
    }
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct SmallData {
    pub id: u64,
    pub status: u8,
    pub enabled: bool,
    pub name: String,
    pub retries: u8,
    pub description: String,
    pub config: std::collections::HashMap<String, u32>,
}

impl SmallData {
    pub fn new() -> Self {
        let long_string = iter::repeat('o').take(250).collect::<String>();

        // Create realistic HashMap with shorter keys.
        let mut config = HashMap::new();
        for i in 0..100 {
            config.insert(format!("cfg_{}", i), i);
        }

        Self {
            id: 42,
            status: 1,
            enabled: true,
            name: "Test Item".to_string(),
            retries: 5,
            description: long_string,
            config,
        }
    }
}
