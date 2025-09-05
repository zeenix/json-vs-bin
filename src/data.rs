use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zvariant::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct BigData<'a> {
    pub user_id: u64,
    pub status_code: u8,
    pub is_active: bool,
    #[serde(borrow)]
    pub name: &'a str,
    pub retry_count: u8,
    #[serde(borrow)]
    pub description: &'a str,
    #[serde(borrow)]
    pub properties: std::collections::HashMap<&'a str, u32>,
    pub priority: u8,
    #[serde(borrow)]
    pub message: &'a str,
    pub sequence_num: u32,
    #[serde(borrow)]
    pub metadata: std::collections::HashMap<&'a str, u32>,
    // Additional fields with realistic names
    pub session_id: u64,
    pub error_code: u8,
    pub is_verified: bool,
    #[serde(borrow)]
    pub title: &'a str,
    pub attempt_count: u8,
    #[serde(borrow)]
    pub content: &'a str,
    #[serde(borrow)]
    pub attributes: std::collections::HashMap<&'a str, u32>,
    pub level: u8,
    #[serde(borrow)]
    pub details: &'a str,
    pub request_id: u32,
    #[serde(borrow)]
    pub headers: std::collections::HashMap<&'a str, u32>,
    pub timestamp: u64,
    pub response_code: u8,
    pub is_complete: bool,
    #[serde(borrow)]
    pub summary: &'a str,
    pub max_retries: u8,
    #[serde(borrow)]
    pub body: &'a str,
    #[serde(borrow)]
    pub tags: std::collections::HashMap<&'a str, u32>,
}

// Static data for benchmarks
static LONG_STRING: &str = "oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo";
static PROP_KEYS: [&str; 100] = [
    "prop_0", "prop_1", "prop_2", "prop_3", "prop_4", "prop_5", "prop_6", "prop_7", "prop_8",
    "prop_9", "prop_10", "prop_11", "prop_12", "prop_13", "prop_14", "prop_15", "prop_16",
    "prop_17", "prop_18", "prop_19", "prop_20", "prop_21", "prop_22", "prop_23", "prop_24",
    "prop_25", "prop_26", "prop_27", "prop_28", "prop_29", "prop_30", "prop_31", "prop_32",
    "prop_33", "prop_34", "prop_35", "prop_36", "prop_37", "prop_38", "prop_39", "prop_40",
    "prop_41", "prop_42", "prop_43", "prop_44", "prop_45", "prop_46", "prop_47", "prop_48",
    "prop_49", "prop_50", "prop_51", "prop_52", "prop_53", "prop_54", "prop_55", "prop_56",
    "prop_57", "prop_58", "prop_59", "prop_60", "prop_61", "prop_62", "prop_63", "prop_64",
    "prop_65", "prop_66", "prop_67", "prop_68", "prop_69", "prop_70", "prop_71", "prop_72",
    "prop_73", "prop_74", "prop_75", "prop_76", "prop_77", "prop_78", "prop_79", "prop_80",
    "prop_81", "prop_82", "prop_83", "prop_84", "prop_85", "prop_86", "prop_87", "prop_88",
    "prop_89", "prop_90", "prop_91", "prop_92", "prop_93", "prop_94", "prop_95", "prop_96",
    "prop_97", "prop_98", "prop_99",
];
static META_KEYS: [&str; 100] = [
    "meta_0", "meta_1", "meta_2", "meta_3", "meta_4", "meta_5", "meta_6", "meta_7", "meta_8",
    "meta_9", "meta_10", "meta_11", "meta_12", "meta_13", "meta_14", "meta_15", "meta_16",
    "meta_17", "meta_18", "meta_19", "meta_20", "meta_21", "meta_22", "meta_23", "meta_24",
    "meta_25", "meta_26", "meta_27", "meta_28", "meta_29", "meta_30", "meta_31", "meta_32",
    "meta_33", "meta_34", "meta_35", "meta_36", "meta_37", "meta_38", "meta_39", "meta_40",
    "meta_41", "meta_42", "meta_43", "meta_44", "meta_45", "meta_46", "meta_47", "meta_48",
    "meta_49", "meta_50", "meta_51", "meta_52", "meta_53", "meta_54", "meta_55", "meta_56",
    "meta_57", "meta_58", "meta_59", "meta_60", "meta_61", "meta_62", "meta_63", "meta_64",
    "meta_65", "meta_66", "meta_67", "meta_68", "meta_69", "meta_70", "meta_71", "meta_72",
    "meta_73", "meta_74", "meta_75", "meta_76", "meta_77", "meta_78", "meta_79", "meta_80",
    "meta_81", "meta_82", "meta_83", "meta_84", "meta_85", "meta_86", "meta_87", "meta_88",
    "meta_89", "meta_90", "meta_91", "meta_92", "meta_93", "meta_94", "meta_95", "meta_96",
    "meta_97", "meta_98", "meta_99",
];
static ATTR_KEYS: [&str; 100] = [
    "attr_0", "attr_1", "attr_2", "attr_3", "attr_4", "attr_5", "attr_6", "attr_7", "attr_8",
    "attr_9", "attr_10", "attr_11", "attr_12", "attr_13", "attr_14", "attr_15", "attr_16",
    "attr_17", "attr_18", "attr_19", "attr_20", "attr_21", "attr_22", "attr_23", "attr_24",
    "attr_25", "attr_26", "attr_27", "attr_28", "attr_29", "attr_30", "attr_31", "attr_32",
    "attr_33", "attr_34", "attr_35", "attr_36", "attr_37", "attr_38", "attr_39", "attr_40",
    "attr_41", "attr_42", "attr_43", "attr_44", "attr_45", "attr_46", "attr_47", "attr_48",
    "attr_49", "attr_50", "attr_51", "attr_52", "attr_53", "attr_54", "attr_55", "attr_56",
    "attr_57", "attr_58", "attr_59", "attr_60", "attr_61", "attr_62", "attr_63", "attr_64",
    "attr_65", "attr_66", "attr_67", "attr_68", "attr_69", "attr_70", "attr_71", "attr_72",
    "attr_73", "attr_74", "attr_75", "attr_76", "attr_77", "attr_78", "attr_79", "attr_80",
    "attr_81", "attr_82", "attr_83", "attr_84", "attr_85", "attr_86", "attr_87", "attr_88",
    "attr_89", "attr_90", "attr_91", "attr_92", "attr_93", "attr_94", "attr_95", "attr_96",
    "attr_97", "attr_98", "attr_99",
];
static HEADER_KEYS: [&str; 100] = [
    "header_0",
    "header_1",
    "header_2",
    "header_3",
    "header_4",
    "header_5",
    "header_6",
    "header_7",
    "header_8",
    "header_9",
    "header_10",
    "header_11",
    "header_12",
    "header_13",
    "header_14",
    "header_15",
    "header_16",
    "header_17",
    "header_18",
    "header_19",
    "header_20",
    "header_21",
    "header_22",
    "header_23",
    "header_24",
    "header_25",
    "header_26",
    "header_27",
    "header_28",
    "header_29",
    "header_30",
    "header_31",
    "header_32",
    "header_33",
    "header_34",
    "header_35",
    "header_36",
    "header_37",
    "header_38",
    "header_39",
    "header_40",
    "header_41",
    "header_42",
    "header_43",
    "header_44",
    "header_45",
    "header_46",
    "header_47",
    "header_48",
    "header_49",
    "header_50",
    "header_51",
    "header_52",
    "header_53",
    "header_54",
    "header_55",
    "header_56",
    "header_57",
    "header_58",
    "header_59",
    "header_60",
    "header_61",
    "header_62",
    "header_63",
    "header_64",
    "header_65",
    "header_66",
    "header_67",
    "header_68",
    "header_69",
    "header_70",
    "header_71",
    "header_72",
    "header_73",
    "header_74",
    "header_75",
    "header_76",
    "header_77",
    "header_78",
    "header_79",
    "header_80",
    "header_81",
    "header_82",
    "header_83",
    "header_84",
    "header_85",
    "header_86",
    "header_87",
    "header_88",
    "header_89",
    "header_90",
    "header_91",
    "header_92",
    "header_93",
    "header_94",
    "header_95",
    "header_96",
    "header_97",
    "header_98",
    "header_99",
];
static TAG_KEYS: [&str; 100] = [
    "tag_0", "tag_1", "tag_2", "tag_3", "tag_4", "tag_5", "tag_6", "tag_7", "tag_8", "tag_9",
    "tag_10", "tag_11", "tag_12", "tag_13", "tag_14", "tag_15", "tag_16", "tag_17", "tag_18",
    "tag_19", "tag_20", "tag_21", "tag_22", "tag_23", "tag_24", "tag_25", "tag_26", "tag_27",
    "tag_28", "tag_29", "tag_30", "tag_31", "tag_32", "tag_33", "tag_34", "tag_35", "tag_36",
    "tag_37", "tag_38", "tag_39", "tag_40", "tag_41", "tag_42", "tag_43", "tag_44", "tag_45",
    "tag_46", "tag_47", "tag_48", "tag_49", "tag_50", "tag_51", "tag_52", "tag_53", "tag_54",
    "tag_55", "tag_56", "tag_57", "tag_58", "tag_59", "tag_60", "tag_61", "tag_62", "tag_63",
    "tag_64", "tag_65", "tag_66", "tag_67", "tag_68", "tag_69", "tag_70", "tag_71", "tag_72",
    "tag_73", "tag_74", "tag_75", "tag_76", "tag_77", "tag_78", "tag_79", "tag_80", "tag_81",
    "tag_82", "tag_83", "tag_84", "tag_85", "tag_86", "tag_87", "tag_88", "tag_89", "tag_90",
    "tag_91", "tag_92", "tag_93", "tag_94", "tag_95", "tag_96", "tag_97", "tag_98", "tag_99",
];
static CFG_KEYS: [&str; 100] = [
    "cfg_0", "cfg_1", "cfg_2", "cfg_3", "cfg_4", "cfg_5", "cfg_6", "cfg_7", "cfg_8", "cfg_9",
    "cfg_10", "cfg_11", "cfg_12", "cfg_13", "cfg_14", "cfg_15", "cfg_16", "cfg_17", "cfg_18",
    "cfg_19", "cfg_20", "cfg_21", "cfg_22", "cfg_23", "cfg_24", "cfg_25", "cfg_26", "cfg_27",
    "cfg_28", "cfg_29", "cfg_30", "cfg_31", "cfg_32", "cfg_33", "cfg_34", "cfg_35", "cfg_36",
    "cfg_37", "cfg_38", "cfg_39", "cfg_40", "cfg_41", "cfg_42", "cfg_43", "cfg_44", "cfg_45",
    "cfg_46", "cfg_47", "cfg_48", "cfg_49", "cfg_50", "cfg_51", "cfg_52", "cfg_53", "cfg_54",
    "cfg_55", "cfg_56", "cfg_57", "cfg_58", "cfg_59", "cfg_60", "cfg_61", "cfg_62", "cfg_63",
    "cfg_64", "cfg_65", "cfg_66", "cfg_67", "cfg_68", "cfg_69", "cfg_70", "cfg_71", "cfg_72",
    "cfg_73", "cfg_74", "cfg_75", "cfg_76", "cfg_77", "cfg_78", "cfg_79", "cfg_80", "cfg_81",
    "cfg_82", "cfg_83", "cfg_84", "cfg_85", "cfg_86", "cfg_87", "cfg_88", "cfg_89", "cfg_90",
    "cfg_91", "cfg_92", "cfg_93", "cfg_94", "cfg_95", "cfg_96", "cfg_97", "cfg_98", "cfg_99",
];

impl<'a> BigData<'a> {
    pub fn new() -> BigData<'static> {
        // Create realistic HashMap with static keys.
        let mut properties = HashMap::new();
        for i in 0..100 {
            properties.insert(PROP_KEYS[i], i as u32);
        }

        // Different HashMap instances with different key patterns to simulate variety.
        let mut metadata = HashMap::new();
        for i in 0..100 {
            metadata.insert(META_KEYS[i], (i * 2) as u32);
        }

        let mut attributes = HashMap::new();
        for i in 0..100 {
            attributes.insert(ATTR_KEYS[i], (i * 3) as u32);
        }

        let mut headers = HashMap::new();
        for i in 0..100 {
            headers.insert(HEADER_KEYS[i], (i * 4) as u32);
        }

        let mut tags = HashMap::new();
        for i in 0..100 {
            tags.insert(TAG_KEYS[i], (i * 5) as u32);
        }

        BigData {
            user_id: 42,
            status_code: 200,
            is_active: true,
            name: "John Doe",
            retry_count: 3,
            description: LONG_STRING,
            properties,
            priority: 1,
            message: "Processing request",
            sequence_num: 12345,
            metadata,
            session_id: 987654321,
            error_code: 0,
            is_verified: true,
            title: "Important Task",
            attempt_count: 2,
            content: LONG_STRING,
            attributes,
            level: 5,
            details: LONG_STRING,
            request_id: 67890,
            headers,
            timestamp: 1640000000,
            response_code: 201,
            is_complete: false,
            summary: "Task in progress",
            max_retries: 10,
            body: LONG_STRING,
            tags,
        }
    }
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct SmallData<'a> {
    pub id: u64,
    pub status: u8,
    pub enabled: bool,
    #[serde(borrow)]
    pub name: &'a str,
    pub retries: u8,
    #[serde(borrow)]
    pub description: &'a str,
    #[serde(borrow)]
    pub config: std::collections::HashMap<&'a str, u32>,
}

impl<'a> SmallData<'a> {
    pub fn new() -> SmallData<'static> {
        // Create realistic HashMap with static keys.
        let mut config = HashMap::new();
        for i in 0..100 {
            config.insert(CFG_KEYS[i], i as u32);
        }

        SmallData {
            id: 42,
            status: 1,
            enabled: true,
            name: "Test Item",
            retries: 5,
            description: LONG_STRING,
            config,
        }
    }
}
