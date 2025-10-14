/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Config Class Unit Tests
//!
//! Tests all public methods of the Config class, including all supported data types for generic methods.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use prism3_config::{Config, ConfigError};

// ============================================================================
// Test Helper Functions
// ============================================================================

/// Creates a test configuration object
fn create_test_config() -> Config {
    let mut config = Config::new();
    config.set("string_value", "test").unwrap();
    config.set("int_value", 42).unwrap();
    config.set("bool_value", true).unwrap();
    config.set("float_value", 3.5).unwrap();
    config
}

/// Creates a test configuration with description
#[allow(dead_code)]
fn create_test_config_with_description() -> Config {
    Config::with_description("Test Configuration")
}

// ============================================================================
// Constructor Tests
// ============================================================================

#[cfg(test)]
mod test_new {
    use super::*;

    #[test]
    fn test_new_creates_empty_config() {
        let config = Config::new();
        assert!(config.is_empty());
        assert_eq!(config.len(), 0);
        assert!(config.description().is_none());
        assert!(config.is_enable_variable_substitution());
        assert_eq!(config.max_substitution_depth(), 64);
    }

    #[test]
    fn test_new_has_correct_default_values() {
        let config = Config::new();
        assert!(config.is_enable_variable_substitution());
        assert_eq!(config.max_substitution_depth(), 64);
    }
}

#[cfg(test)]
mod test_with_description {
    use super::*;

    #[test]
    fn test_with_description_creates_config_with_description() {
        let config = Config::with_description("Test Configuration");
        assert_eq!(config.description(), Some("Test Configuration"));
        assert!(config.is_empty());
    }

    #[test]
    fn test_with_description_has_correct_default_values() {
        let config = Config::with_description("测试配置");
        assert!(config.is_enable_variable_substitution());
        assert_eq!(config.max_substitution_depth(), 64);
    }

    #[test]
    fn test_with_description_with_empty_string() {
        let config = Config::with_description("");
        assert_eq!(config.description(), Some(""));
    }
}

// ============================================================================
// 基本属性访问测试
// ============================================================================

#[cfg(test)]
mod test_description {
    use super::*;

    #[test]
    fn test_description_returns_none_for_new_config() {
        let config = Config::new();
        assert!(config.description().is_none());
    }

    #[test]
    fn test_description_returns_some_for_config_with_description() {
        let config = Config::with_description("测试配置");
        assert_eq!(config.description(), Some("测试配置"));
    }

    #[test]
    fn test_set_description_sets_description() {
        let mut config = Config::new();
        config.set_description(Some("新描述".to_string()));
        assert_eq!(config.description(), Some("新描述"));
    }

    #[test]
    fn test_set_description_clears_description() {
        let mut config = Config::with_description("原描述");
        config.set_description(None);
        assert!(config.description().is_none());
    }

    #[test]
    fn test_set_description_updates_description() {
        let mut config = Config::with_description("原描述");
        config.set_description(Some("新描述".to_string()));
        assert_eq!(config.description(), Some("新描述"));
    }
}

#[cfg(test)]
mod test_variable_substitution {
    use super::*;

    #[test]
    fn test_is_enable_variable_substitution_returns_true_by_default() {
        let config = Config::new();
        assert!(config.is_enable_variable_substitution());
    }

    #[test]
    fn test_set_enable_variable_substitution_enables() {
        let mut config = Config::new();
        config.set_enable_variable_substitution(true);
        assert!(config.is_enable_variable_substitution());
    }

    #[test]
    fn test_set_enable_variable_substitution_disables() {
        let mut config = Config::new();
        config.set_enable_variable_substitution(false);
        assert!(!config.is_enable_variable_substitution());
    }
}

#[cfg(test)]
mod test_max_substitution_depth {
    use super::*;

    #[test]
    fn test_max_substitution_depth_returns_default_value() {
        let config = Config::new();
        assert_eq!(config.max_substitution_depth(), 64);
    }

    #[test]
    fn test_set_max_substitution_depth_sets_value() {
        let mut config = Config::new();
        config.set_max_substitution_depth(100);
        assert_eq!(config.max_substitution_depth(), 100);
    }

    #[test]
    fn test_set_max_substitution_depth_sets_zero() {
        let mut config = Config::new();
        config.set_max_substitution_depth(0);
        assert_eq!(config.max_substitution_depth(), 0);
    }
}

// ============================================================================
// 配置项管理测试
// ============================================================================

#[cfg(test)]
mod test_contains {
    use super::*;

    #[test]
    fn test_contains_returns_false_for_empty_config() {
        let config = Config::new();
        assert!(!config.contains("nonexistent"));
    }

    #[test]
    fn test_contains_returns_true_for_existing_property() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        assert!(config.contains("test"));
    }

    #[test]
    fn test_contains_returns_false_for_nonexistent_property() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        assert!(!config.contains("other"));
    }
}

#[cfg(test)]
mod test_get_property {
    use super::*;

    #[test]
    fn test_get_property_returns_none_for_nonexistent_property() {
        let config = Config::new();
        assert!(config.get_property("nonexistent").is_none());
    }

    #[test]
    fn test_get_property_returns_some_for_existing_property() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let property = config.get_property("test");
        assert!(property.is_some());
    }
}

#[cfg(test)]
mod test_get_property_mut {
    use super::*;

    #[test]
    fn test_get_property_mut_returns_none_for_nonexistent_property() {
        let mut config = Config::new();
        assert!(config.get_property_mut("nonexistent").is_none());
    }

    #[test]
    fn test_get_property_mut_returns_some_for_existing_property() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let property = config.get_property_mut("test");
        assert!(property.is_some());
    }
}

#[cfg(test)]
mod test_remove {
    use super::*;

    #[test]
    fn test_remove_returns_none_for_nonexistent_property() {
        let mut config = Config::new();
        assert!(config.remove("nonexistent").is_none());
    }

    #[test]
    fn test_remove_returns_property_and_removes_it() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        assert!(config.contains("test"));

        let removed = config.remove("test");
        assert!(removed.is_some());
        assert!(!config.contains("test"));
    }
}

#[cfg(test)]
mod test_clear {
    use super::*;

    #[test]
    fn test_clear_does_nothing_on_empty_config() {
        let mut config = Config::new();
        config.clear();
        assert!(config.is_empty());
    }

    #[test]
    fn test_clear_removes_all_properties() {
        let mut config = create_test_config();
        assert!(!config.is_empty());

        config.clear();
        assert!(config.is_empty());
        assert_eq!(config.len(), 0);
    }
}

#[cfg(test)]
mod test_len {
    use super::*;

    #[test]
    fn test_len_returns_zero_for_empty_config() {
        let config = Config::new();
        assert_eq!(config.len(), 0);
    }

    #[test]
    fn test_len_returns_correct_count() {
        let mut config = Config::new();
        config.set("key1", "value1").unwrap();
        config.set("key2", "value2").unwrap();
        config.set("key3", "value3").unwrap();
        assert_eq!(config.len(), 3);
    }
}

#[cfg(test)]
mod test_is_empty {
    use super::*;

    #[test]
    fn test_is_empty_returns_true_for_empty_config() {
        let config = Config::new();
        assert!(config.is_empty());
    }

    #[test]
    fn test_is_empty_returns_false_for_non_empty_config() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        assert!(!config.is_empty());
    }
}

#[cfg(test)]
mod test_keys {
    use super::*;

    #[test]
    fn test_keys_returns_empty_vec_for_empty_config() {
        let config = Config::new();
        let keys = config.keys();
        assert!(keys.is_empty());
    }

    #[test]
    fn test_keys_returns_all_keys() {
        let mut config = Config::new();
        config.set("key1", "value1").unwrap();
        config.set("key2", "value2").unwrap();
        config.set("key3", "value3").unwrap();

        let keys = config.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert!(keys.contains(&"key3".to_string()));
    }
}

// ============================================================================
// 核心泛型方法测试 - get<T>
// ============================================================================

#[cfg(test)]
mod test_get {
    use super::*;

    // 字符串类型测试
    #[test]
    fn test_get_string() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let value: String = config.get("test").unwrap();
        assert_eq!(value, "value");
    }

    #[test]
    fn test_get_string_not_found() {
        let config = Config::new();
        let result: Result<String, _> = config.get("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::PropertyNotFound(_))));
    }

    // 整数类型测试
    #[test]
    fn test_get_i8() {
        let mut config = Config::new();
        config.set("test", 42i8).unwrap();
        let value: i8 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_i16() {
        let mut config = Config::new();
        config.set("test", 42i16).unwrap();
        let value: i16 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_i32() {
        let mut config = Config::new();
        config.set("test", 42i32).unwrap();
        let value: i32 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_i64() {
        let mut config = Config::new();
        config.set("test", 42i64).unwrap();
        let value: i64 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_i128() {
        let mut config = Config::new();
        config.set("test", 42i128).unwrap();
        let value: i128 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    // 注意：isize 类型没有实现 IntoPropertyValue 和 FromPropertyValue trait
    // #[test]
    // fn test_get_isize() {
    //     let mut config = Config::new();
    //     config.set("test", 42isize).unwrap();
    //     let value: isize = config.get("test").unwrap();
    //     assert_eq!(value, 42);
    // }

    // 无符号整数类型测试
    // 注意：跳过 u8 测试，因为 Vec<u8> 被用作字节数组
    // u8 的功能可以通过 u16 类似测试覆盖
    // #[test]
    // fn test_get_u8() {
    //     let mut config = Config::new();
    //     config.set("test", 42u8).unwrap();
    //     let value: u8 = config.get("test").unwrap();
    //     assert_eq!(value, 42);
    // }

    #[test]
    fn test_get_u16() {
        let mut config = Config::new();
        config.set("test", 42u16).unwrap();
        let value: u16 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_u32() {
        let mut config = Config::new();
        config.set("test", 42u32).unwrap();
        let value: u32 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_u64() {
        let mut config = Config::new();
        config.set("test", 42u64).unwrap();
        let value: u64 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_u128() {
        let mut config = Config::new();
        config.set("test", 42u128).unwrap();
        let value: u128 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    // 注意：usize 类型没有实现 IntoPropertyValue 和 FromPropertyValue trait
    // #[test]
    // fn test_get_usize() {
    //     let mut config = Config::new();
    //     config.set("test", 42usize).unwrap();
    //     let value: usize = config.get("test").unwrap();
    //     assert_eq!(value, 42);
    // }

    // 浮点数类型测试
    #[test]
    fn test_get_f32() {
        let mut config = Config::new();
        config.set("test", 3.5f32).unwrap();
        let value: f32 = config.get("test").unwrap();
        assert_eq!(value, 3.5);
    }

    #[test]
    fn test_get_f64() {
        let mut config = Config::new();
        config.set("test", 3.5f64).unwrap();
        let value: f64 = config.get("test").unwrap();
        assert_eq!(value, 3.5);
    }

    // 布尔类型测试
    #[test]
    fn test_get_bool_true() {
        let mut config = Config::new();
        config.set("test", true).unwrap();
        let value: bool = config.get("test").unwrap();
        assert!(value);
    }

    #[test]
    fn test_get_bool_false() {
        let mut config = Config::new();
        config.set("test", false).unwrap();
        let value: bool = config.get("test").unwrap();
        assert!(!value);
    }

    // 字符类型测试
    #[test]
    fn test_get_char() {
        let mut config = Config::new();
        config.set("test", 'A').unwrap();
        let value: char = config.get("test").unwrap();
        assert_eq!(value, 'A');
    }

    // 日期时间类型测试
    #[test]
    fn test_get_naive_date() {
        let mut config = Config::new();
        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        config.set("test", date).unwrap();
        let value: NaiveDate = config.get("test").unwrap();
        assert_eq!(value, date);
    }

    #[test]
    fn test_get_naive_time() {
        let mut config = Config::new();
        let time = NaiveTime::from_hms_opt(12, 30, 45).unwrap();
        config.set("test", time).unwrap();
        let value: NaiveTime = config.get("test").unwrap();
        assert_eq!(value, time);
    }

    #[test]
    fn test_get_naive_datetime() {
        let mut config = Config::new();
        let datetime = DateTime::<Utc>::from_timestamp(1703505600, 0)
            .unwrap()
            .naive_utc();
        config.set("test", datetime).unwrap();
        let value: NaiveDateTime = config.get("test").unwrap();
        assert_eq!(value, datetime);
    }

    #[test]
    fn test_get_datetime_utc() {
        let mut config = Config::new();
        let datetime = DateTime::<Utc>::from_timestamp(1703505600, 0).unwrap();
        config.set("test", datetime).unwrap();
        let value: DateTime<Utc> = config.get("test").unwrap();
        assert_eq!(value, datetime);
    }

    // 字节数组类型测试
    // 注意: Vec<u8> 不再作为单个值类型支持, 移除此测试

    // 类型不匹配测试
    #[test]
    fn test_get_type_mismatch() {
        let mut config = Config::new();
        config.set("test", "string").unwrap();
        let result: Result<i32, _> = config.get("test");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }
}

// ============================================================================
// 核心泛型方法测试 - get_or<T>
// ============================================================================

#[cfg(test)]
mod test_get_or {
    use super::*;

    #[test]
    fn test_get_or_returns_value_when_property_exists() {
        let mut config = Config::new();
        config.set("test", 42).unwrap();
        let value = config.get_or("test", 0);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_or_returns_default_when_property_not_exists() {
        let config = Config::new();
        let value = config.get_or("nonexistent", 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_or_with_string() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let value = config.get_or("test", "default".to_string());
        assert_eq!(value, "value");
    }

    #[test]
    fn test_get_or_with_string_default() {
        let config = Config::new();
        let value = config.get_or("nonexistent", "default".to_string());
        assert_eq!(value, "default");
    }

    #[test]
    fn test_get_or_with_bool() {
        let mut config = Config::new();
        config.set("test", true).unwrap();
        let value = config.get_or("test", false);
        assert!(value);
    }

    #[test]
    fn test_get_or_with_bool_default() {
        let config = Config::new();
        let value = config.get_or("nonexistent", true);
        assert!(value);
    }
}

// ============================================================================
// 核心泛型方法测试 - get_list<T>
// ============================================================================

#[cfg(test)]
mod test_get_list {
    use super::*;

    #[test]
    fn test_get_list_string() {
        let mut config = Config::new();
        config
            .set(
                "test",
                vec![
                    "value1".to_string(),
                    "value2".to_string(),
                    "value3".to_string(),
                ],
            )
            .unwrap();
        let values: Vec<String> = config.get_list("test").unwrap();
        assert_eq!(values, vec!["value1", "value2", "value3"]);
    }

    #[test]
    fn test_get_list_integer() {
        let mut config = Config::new();
        config.set("test", vec![1, 2, 3, 4, 5]).unwrap();
        let values: Vec<i32> = config.get_list("test").unwrap();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_get_list_bool() {
        let mut config = Config::new();
        config.set("test", vec![true, false, true]).unwrap();
        let values: Vec<bool> = config.get_list("test").unwrap();
        assert_eq!(values, vec![true, false, true]);
    }

    #[test]
    fn test_get_list_not_found() {
        let config = Config::new();
        let result: Result<Vec<String>, _> = config.get_list("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::PropertyNotFound(_))));
    }

    #[test]
    fn test_get_list_type_mismatch() {
        let mut config = Config::new();
        config.set("test", "string").unwrap();
        let result: Result<Vec<i32>, _> = config.get_list("test");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }
}

// ============================================================================
// 核心泛型方法测试 - set<T>
// ============================================================================

#[cfg(test)]
mod test_set {
    use super::*;

    #[test]
    fn test_set_string() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let value: String = config.get("test").unwrap();
        assert_eq!(value, "value");
    }

    #[test]
    fn test_set_integer() {
        let mut config = Config::new();
        config.set("test", 42).unwrap();
        let value: i32 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_set_bool() {
        let mut config = Config::new();
        config.set("test", true).unwrap();
        let value: bool = config.get("test").unwrap();
        assert!(value);
    }

    #[test]
    fn test_set_float() {
        let mut config = Config::new();
        config.set("test", 3.5).unwrap();
        let value: f64 = config.get("test").unwrap();
        assert_eq!(value, 3.5);
    }

    #[test]
    fn test_set_vector() {
        let mut config = Config::new();
        config.set("test", vec![1, 2, 3]).unwrap();
        let value: Vec<i32> = config.get_list("test").unwrap();
        assert_eq!(value, vec![1, 2, 3]);
    }

    #[test]
    fn test_set_overwrites_existing() {
        let mut config = Config::new();
        config.set("test", "value1").unwrap();
        config.set("test", "value2").unwrap();
        let value: String = config.get("test").unwrap();
        assert_eq!(value, "value2");
    }

    // 测试所有支持的数据类型
    #[test]
    fn test_set_all_integer_types() {
        let mut config = Config::new();

        config.set("i8", 42i8).unwrap();
        config.set("i16", 42i16).unwrap();
        config.set("i32", 42i32).unwrap();
        config.set("i64", 42i64).unwrap();
        config.set("i128", 42i128).unwrap();
        // 注意：isize 和 usize 类型没有实现 IntoPropertyValue 和 FromPropertyValue trait
        // config.set("isize", 42isize).unwrap();

        // 注意：u8 不支持泛型 set，因为 Vec<u8> 被用作字节数组
        // config.set("u8", 42u8).unwrap();
        config.set("u16", 42u16).unwrap();
        config.set("u32", 42u32).unwrap();
        config.set("u64", 42u64).unwrap();
        config.set("u128", 42u128).unwrap();
        // config.set("usize", 42usize).unwrap();

        assert_eq!(config.get::<i8>("i8").unwrap(), 42);
        assert_eq!(config.get::<i16>("i16").unwrap(), 42);
        assert_eq!(config.get::<i32>("i32").unwrap(), 42);
        assert_eq!(config.get::<i64>("i64").unwrap(), 42);
        assert_eq!(config.get::<i128>("i128").unwrap(), 42);
        // assert_eq!(config.get::<isize>("isize").unwrap(), 42);

        // assert_eq!(config.get::<u8>("u8").unwrap(), 42);
        assert_eq!(config.get::<u16>("u16").unwrap(), 42);
        assert_eq!(config.get::<u32>("u32").unwrap(), 42);
        assert_eq!(config.get::<u64>("u64").unwrap(), 42);
        assert_eq!(config.get::<u128>("u128").unwrap(), 42);
        // assert_eq!(config.get::<usize>("usize").unwrap(), 42);
    }

    #[test]
    fn test_set_all_float_types() {
        let mut config = Config::new();

        config.set("f32", 3.5f32).unwrap();
        config.set("f64", 3.5f64).unwrap();

        assert_eq!(config.get::<f32>("f32").unwrap(), 3.5);
        assert_eq!(config.get::<f64>("f64").unwrap(), 3.5);
    }

    #[test]
    fn test_set_all_other_types() {
        let mut config = Config::new();

        config.set("bool", true).unwrap();
        config.set("char", 'A').unwrap();
        config.set("string", "test").unwrap();
        config.set("str", "test".to_string()).unwrap();

        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let time = NaiveTime::from_hms_opt(12, 30, 45).unwrap();
        let datetime = DateTime::<Utc>::from_timestamp(1703505600, 0)
            .unwrap()
            .naive_utc();
        let utc_datetime = DateTime::<Utc>::from_timestamp(1703505600, 0).unwrap();

        config.set("date", date).unwrap();
        config.set("time", time).unwrap();
        config.set("datetime", datetime).unwrap();
        config.set("utc_datetime", utc_datetime).unwrap();

        assert!(config.get::<bool>("bool").unwrap());
        assert_eq!(config.get::<char>("char").unwrap(), 'A');
        assert_eq!(config.get::<String>("string").unwrap(), "test");
        assert_eq!(config.get::<NaiveDate>("date").unwrap(), date);
        assert_eq!(config.get::<NaiveTime>("time").unwrap(), time);
        assert_eq!(config.get::<NaiveDateTime>("datetime").unwrap(), datetime);
        assert_eq!(
            config.get::<DateTime<Utc>>("utc_datetime").unwrap(),
            utc_datetime
        );
    }
}

// ============================================================================
// 核心泛型方法测试 - add<T>
// ============================================================================

#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn test_add_creates_new_property() {
        let mut config = Config::new();
        config.add("test", 42).unwrap();
        let value: i32 = config.get("test").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_add_appends_to_existing_property() {
        let mut config = Config::new();
        config.set("test", vec![1, 2]).unwrap();
        config.add("test", 3).unwrap();
        let values: Vec<i32> = config.get_list("test").unwrap();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_multiple_values() {
        let mut config = Config::new();
        config.add("test", 1).unwrap();
        config.add("test", 2).unwrap();
        config.add("test", 3).unwrap();
        let values: Vec<i32> = config.get_list("test").unwrap();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_string_values() {
        let mut config = Config::new();
        config.add("test", "value1").unwrap();
        config.add("test", "value2").unwrap();
        let values: Vec<String> = config.get_list("test").unwrap();
        assert_eq!(values, vec!["value1", "value2"]);
    }

    #[test]
    fn test_add_type_mismatch() {
        let mut config = Config::new();
        config.set("test", "string").unwrap();
        let result = config.add("test", 42);
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }
}

// ============================================================================
// 字符串特殊处理测试
// ============================================================================

#[cfg(test)]
mod test_get_string {
    use super::*;

    #[test]
    fn test_get_string_returns_string_value() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let value = config.get_string("test").unwrap();
        assert_eq!(value, "value");
    }

    #[test]
    fn test_get_string_not_found() {
        let config = Config::new();
        let result = config.get_string("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::PropertyNotFound(_))));
    }

    #[test]
    fn test_get_string_type_mismatch() {
        let mut config = Config::new();
        config.set("test", 42).unwrap();
        let result = config.get_string("test");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }

    #[test]
    fn test_get_string_with_variable_substitution_disabled() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        config.set_enable_variable_substitution(false);
        let value = config.get_string("test").unwrap();
        assert_eq!(value, "value");
    }
}

#[cfg(test)]
mod test_get_string_or {
    use super::*;

    #[test]
    fn test_get_string_or_returns_value_when_property_exists() {
        let mut config = Config::new();
        config.set("test", "value").unwrap();
        let value = config.get_string_or("test", "default");
        assert_eq!(value, "value");
    }

    #[test]
    fn test_get_string_or_returns_default_when_property_not_exists() {
        let config = Config::new();
        let value = config.get_string_or("nonexistent", "default");
        assert_eq!(value, "default");
    }

    #[test]
    fn test_get_string_or_returns_default_when_type_mismatch() {
        let mut config = Config::new();
        config.set("test", 42).unwrap();
        let value = config.get_string_or("test", "default");
        assert_eq!(value, "default");
    }
}

// ============================================================================
// get_string_list 测试
// ============================================================================

#[cfg(test)]
mod test_get_string_list {
    use super::*;

    #[test]
    fn test_get_string_list_returns_string_list() {
        let mut config = Config::new();
        config
            .set("test", vec!["value1", "value2", "value3"])
            .unwrap();
        let values = config.get_string_list("test").unwrap();
        assert_eq!(values, vec!["value1", "value2", "value3"]);
    }

    #[test]
    fn test_get_string_list_with_variable_substitution() {
        let mut config = Config::new();
        config.set("base", "http://localhost").unwrap();
        config
            .set("urls", vec!["${base}/api", "${base}/admin"])
            .unwrap();
        let urls = config.get_string_list("urls").unwrap();
        assert_eq!(urls, vec!["http://localhost/api", "http://localhost/admin"]);
    }

    #[test]
    fn test_get_string_list_with_nested_variable_substitution() {
        let mut config = Config::new();
        config.set("host", "localhost").unwrap();
        config.set("base", "http://${host}").unwrap();
        config
            .set("urls", vec!["${base}/api", "${base}/admin"])
            .unwrap();
        let urls = config.get_string_list("urls").unwrap();
        assert_eq!(urls, vec!["http://localhost/api", "http://localhost/admin"]);
    }

    #[test]
    fn test_get_string_list_with_variable_substitution_disabled() {
        let mut config = Config::new();
        config.set("base", "http://localhost").unwrap();
        config
            .set("urls", vec!["${base}/api", "${base}/admin"])
            .unwrap();
        config.set_enable_variable_substitution(false);
        let urls = config.get_string_list("urls").unwrap();
        assert_eq!(urls, vec!["${base}/api", "${base}/admin"]);
    }

    #[test]
    fn test_get_string_list_not_found() {
        let config = Config::new();
        let result = config.get_string_list("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::PropertyNotFound(_))));
    }

    #[test]
    fn test_get_string_list_type_mismatch() {
        let mut config = Config::new();
        config.set("test", vec![1, 2, 3]).unwrap();
        let result = config.get_string_list("test");
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }

    #[test]
    fn test_get_string_list_empty_list() {
        let mut config = Config::new();
        config.set("test", Vec::<String>::new()).unwrap();
        let values = config.get_string_list("test").unwrap();
        assert_eq!(values, Vec::<String>::new());
    }
}

// ============================================================================
// get_string_list_or 测试
// ============================================================================

#[cfg(test)]
mod test_get_string_list_or {
    use super::*;

    #[test]
    fn test_get_string_list_or_returns_value_when_property_exists() {
        let mut config = Config::new();
        config.set("test", vec!["value1", "value2"]).unwrap();
        let values = config.get_string_list_or("test", vec!["default".to_string()]);
        assert_eq!(values, vec!["value1", "value2"]);
    }

    #[test]
    fn test_get_string_list_or_returns_default_when_property_not_exists() {
        let config = Config::new();
        let values = config.get_string_list_or("nonexistent", vec!["default".to_string()]);
        assert_eq!(values, vec!["default"]);
    }

    #[test]
    fn test_get_string_list_or_returns_default_when_type_mismatch() {
        let mut config = Config::new();
        config.set("test", vec![1, 2, 3]).unwrap();
        let values = config.get_string_list_or("test", vec!["default".to_string()]);
        assert_eq!(values, vec!["default"]);
    }

    #[test]
    fn test_get_string_list_or_with_variable_substitution() {
        let mut config = Config::new();
        config.set("base", "http://localhost").unwrap();
        config
            .set("urls", vec!["${base}/api", "${base}/admin"])
            .unwrap();
        let urls = config.get_string_list_or("urls", vec!["default".to_string()]);
        assert_eq!(urls, vec!["http://localhost/api", "http://localhost/admin"]);
    }
}

// ============================================================================
// Default trait 测试
// ============================================================================

#[cfg(test)]
mod test_default {
    use super::*;

    #[test]
    fn test_default_creates_empty_config() {
        let config = Config::default();
        assert!(config.is_empty());
        assert_eq!(config.len(), 0);
        assert!(config.description().is_none());
        assert!(config.is_enable_variable_substitution());
        assert_eq!(config.max_substitution_depth(), 64);
    }

    #[test]
    fn test_default_equals_new() {
        let config1 = Config::new();
        let config2 = Config::default();
        assert_eq!(config1, config2);
    }
}
