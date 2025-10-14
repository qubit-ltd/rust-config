/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # 配置工具函数测试
//!
//! 测试配置工具函数的功能，包括变量替换等。
//!
//! # 作者
//!
//! 胡海星

use prism3_config::{substitute_variables, Config, ConfigError};

#[test]
fn test_substitute_simple() {
    let mut config = Config::new();
    config.set("name", "world").unwrap();

    let result = substitute_variables("Hello, ${name}!", &config, 10).unwrap();
    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_substitute_multiple() {
    let mut config = Config::new();
    config.set("host", "localhost").unwrap();
    config.set("port", "8080").unwrap();

    let result = substitute_variables("http://${host}:${port}/api", &config, 10).unwrap();
    assert_eq!(result, "http://localhost:8080/api");
}

#[test]
fn test_substitute_recursive() {
    let mut config = Config::new();
    config.set("a", "value_a").unwrap();
    config.set("b", "${a}_b").unwrap();
    config.set("c", "${b}_c").unwrap();

    let result = substitute_variables("${c}", &config, 10).unwrap();
    assert_eq!(result, "value_a_b_c");
}

#[test]
fn test_substitute_depth_exceeded() {
    let mut config = Config::new();
    config.set("a", "${b}").unwrap();
    config.set("b", "${a}").unwrap();

    let result = substitute_variables("${a}", &config, 5);
    assert!(matches!(
        result,
        Err(ConfigError::SubstitutionDepthExceeded(5))
    ));
}

#[test]
fn test_substitute_env_var() {
    std::env::set_var("TEST_VAR", "test_value");

    let config = Config::new();
    let result = substitute_variables("Value: ${TEST_VAR}", &config, 10).unwrap();
    assert_eq!(result, "Value: test_value");

    std::env::remove_var("TEST_VAR");
}
