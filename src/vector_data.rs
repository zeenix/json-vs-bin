use serde::{Deserialize, Serialize};
use zvariant::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct SensorReading {
    pub timestamp: u64,
    pub sensor_id: u32,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub battery: u8,
    pub status: u8,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct MarketTick {
    pub timestamp: u64,
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: u64,
    pub bid_size: u32,
    pub ask_size: u32,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct LogEvent {
    pub timestamp: u64,
    pub level: u8,
    pub component: String,
    pub message: String,
    pub trace_id: u64,
    pub span_id: u64,
    pub user_id: u32,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct Metadata {
    pub version: u16,
    pub source: String,
    pub created_at: u64,
    pub batch_id: u64,
    pub compression: bool,
    pub checksum: u32,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct Summary {
    pub count: u32,
    pub min_temp: f32,
    pub max_temp: f32,
    pub avg_temp: f32,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct BigVectorData {
    pub sensors: Vec<SensorReading>,
    pub market: Vec<MarketTick>,
    pub logs: Vec<LogEvent>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct SmallVectorData {
    pub readings: Vec<SensorReading>,
    pub summary: Summary,
}

impl BigVectorData {
    pub fn new() -> Self {
        let base_timestamp = 1_700_000_000_000_000u64;

        let sensors: Vec<SensorReading> = (0..1000)
            .map(|i| SensorReading {
                timestamp: base_timestamp + (i as u64 * 1000),
                sensor_id: (i % 100) as u32,
                temperature: 20.0 + (i as f32 * 0.01).sin() * 5.0,
                humidity: 50.0 + (i as f32 * 0.02).cos() * 20.0,
                pressure: 1013.25 + (i as f32 * 0.015).sin() * 10.0,
                battery: ((100.0 - (i as f32 * 0.01)) as u8).min(100),
                status: (i % 4) as u8,
            })
            .collect();

        let symbols = vec![
            "AAPL", "GOOGL", "MSFT", "AMZN", "META", "TSLA", "NVDA", "AMD",
        ];
        let market: Vec<MarketTick> = (0..500)
            .map(|i| {
                let base_price = 100.0 + (i as f64 * 0.1);
                MarketTick {
                    timestamp: base_timestamp + (i as u64 * 100),
                    symbol: symbols[i % symbols.len()].to_string(),
                    bid: base_price - 0.01,
                    ask: base_price + 0.01,
                    last: base_price,
                    volume: (1000000 + i * 1000) as u64,
                    bid_size: (100 + (i % 50)) as u32,
                    ask_size: (100 + ((i + 25) % 50)) as u32,
                }
            })
            .collect();

        let components = vec!["auth", "db", "api", "cache", "queue", "worker"];
        let messages = vec![
            "Request processed successfully",
            "Connection established",
            "Cache miss, fetching from database",
            "Task queued for processing",
            "Rate limit exceeded",
            "Transaction committed",
        ];

        let logs: Vec<LogEvent> = (0..200)
            .map(|i| LogEvent {
                timestamp: base_timestamp + (i as u64 * 5000),
                level: (i % 4) as u8,
                component: components[i % components.len()].to_string(),
                message: messages[i % messages.len()].to_string(),
                trace_id: 1000000 + (i as u64),
                span_id: 2000000 + (i as u64 * 2),
                user_id: if i % 3 == 0 { (1000 + i) as u32 } else { 0 },
            })
            .collect();

        Self {
            sensors,
            market,
            logs,
            metadata: Metadata {
                version: 1,
                source: "benchmark-system".to_string(),
                created_at: base_timestamp,
                batch_id: 1234567890,
                compression: false,
                checksum: 0x12345678,
            },
        }
    }
}

impl SmallVectorData {
    pub fn new() -> Self {
        let base_timestamp = 1_700_000_000_000_000u64;

        let readings: Vec<SensorReading> = (0..100)
            .map(|i| SensorReading {
                timestamp: base_timestamp + (i as u64 * 1000),
                sensor_id: (i % 10) as u32,
                temperature: 20.0 + (i as f32 * 0.1).sin() * 5.0,
                humidity: 50.0 + (i as f32 * 0.2).cos() * 20.0,
                pressure: 1013.25 + (i as f32 * 0.15).sin() * 10.0,
                battery: ((100.0 - (i as f32 * 0.5)) as u8).min(100),
                status: (i % 4) as u8,
            })
            .collect();

        let temps: Vec<f32> = readings.iter().map(|r| r.temperature).collect();
        let min_temp = temps.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_temp = temps.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg_temp = temps.iter().sum::<f32>() / temps.len() as f32;

        Self {
            readings,
            summary: Summary {
                count: 100,
                min_temp,
                max_temp,
                avg_temp,
                min_timestamp: base_timestamp,
                max_timestamp: base_timestamp + 99_000,
            },
        }
    }
}
