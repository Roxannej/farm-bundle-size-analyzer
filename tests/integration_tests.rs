//! 集成测试 - 测试插件在真实 Farm.js 环境中的行为

use farm_bundle_size_analyzer::FarmBundleSizeAnalyzerPlugin;
use farmfe_core::{
    config::Config,
    context::CompilationContext,
    plugin::Plugin,
    resource::Resource,
};
use std::collections::HashMap;
use std::sync::Arc;

/// 创建模拟的 CompilationContext 用于集成测试
fn create_mock_context() -> Arc<CompilationContext> {
    let mut config = Config::default();
    config.root = std::env::temp_dir().join("farm_integration_test").to_string_lossy().to_string();
    
    Arc::new(CompilationContext {
        config: Box::new(config),
        resources_map: Box::new(farmfe_core::parking_lot::Mutex::new(HashMap::new())),
        ..Default::default()
    })
}

/// 创建模拟的 Resource
fn create_mock_resource(size: usize, content_type: &str) -> Resource {
    let mut resource = Resource {
        bytes: vec![0u8; size],
        ..Default::default()
    };
    
    // 模拟设置资源类型
    if content_type == "js" {
        // 模拟 JS 文件内容
        resource.bytes = b"console.log('Hello World');".repeat(size / 25);
    } else if content_type == "css" {
        // 模拟 CSS 文件内容
        resource.bytes = b"body { margin: 0; padding: 0; }".repeat(size / 30);
    }
    
    resource
}

#[test]
fn test_plugin_integration_with_realistic_bundle() {
    let config = Config::default();
    let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
    let context = create_mock_context();
    
    // 模拟一个真实的 web 应用 bundle
    {
        let mut resources_map = context.resources_map.lock();
        
        // 主 JS 文件
        resources_map.insert(
            "main.js".to_string(),
            create_mock_resource(245 * 1024, "js"), // 245KB
        );
        
        // 样式文件
        resources_map.insert(
            "styles.css".to_string(),
            create_mock_resource(82 * 1024, "css"), // 82KB
        );
        
        // 小文件
        resources_map.insert(
            "index.html".to_string(),
            create_mock_resource(12 * 1024, "html"), // 12KB
        );
        
        // 图片资源
        resources_map.insert(
            "logo.png".to_string(),
            create_mock_resource(45 * 1024, "image"), // 45KB
        );
    }
    
    // 测试插件执行
    let result = plugin.generate_end(&context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

#[test]
fn test_plugin_with_large_files_warning() {
    let config = Config::default();
    let custom_options = r#"{
        "warning_threshold": 100000,
        "show_suggestions": true
    }"#;
    
    let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, custom_options.to_string());
    let context = create_mock_context();
    
    // 添加超过阈值的大文件
    {
        let mut resources_map = context.resources_map.lock();
        resources_map.insert(
            "huge-bundle.js".to_string(),
            create_mock_resource(200 * 1024, "js"), // 200KB > 100KB
        );
        resources_map.insert(
            "small-file.js".to_string(),
            create_mock_resource(50 * 1024, "js"), // 50KB < 100KB
        );
    }
    
    let result = plugin.generate_end(&context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

#[test]
fn test_plugin_with_different_file_types() {
    let config = Config::default();
    let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
    let context = create_mock_context();
    
    // 测试不同类型的文件
    {
        let mut resources_map = context.resources_map.lock();
        
        resources_map.insert("app.js".to_string(), create_mock_resource(1024, "js"));
        resources_map.insert("vendor.js".to_string(), create_mock_resource(2048, "js"));
        resources_map.insert("main.css".to_string(), create_mock_resource(512, "css"));
        resources_map.insert("bootstrap.css".to_string(), create_mock_resource(1536, "css"));
        resources_map.insert("index.html".to_string(), create_mock_resource(256, "html"));
        resources_map.insert("favicon.ico".to_string(), create_mock_resource(128, "image"));
    }
    
    let result = plugin.generate_end(&context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

#[test]
fn test_plugin_performance_with_many_files() {
    let config = Config::default();
    let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
    let context = create_mock_context();
    
    // 测试大量文件的性能
    {
        let mut resources_map = context.resources_map.lock();
        
        for i in 0..100 {
            let filename = format!("chunk_{}.js", i);
            let size = 1024 + (i * 100); // 不同大小的文件
            resources_map.insert(filename, create_mock_resource(size, "js"));
        }
    }
    
    let result = plugin.generate_end(&context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

#[test]
fn test_plugin_error_handling() {
    let config = Config::default();
    let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
    let context = create_mock_context();
    
    // 测试空 bundle 的情况
    let result = plugin.generate_end(&context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

#[test]
fn test_plugin_config_validation() {
    let config = Config::default();
    
    // 测试各种配置组合
    let test_configs = vec![
        ("{}", "空配置"),
        (r#"{"warning_threshold": 0}"#, "零阈值"),
        (r#"{"warning_threshold": 999999999}"#, "极大阈值"),
        (r#"{"show_suggestions": false}"#, "关闭建议"),
        (r#"{"generate_report": true}"#, "开启报告"),
    ];
    
    for (config_json, description) in test_configs {
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, config_json.to_string());
        assert_eq!(plugin.name(), "farm-bundle-size-analyzer", "测试配置: {}", description);
    }
}
