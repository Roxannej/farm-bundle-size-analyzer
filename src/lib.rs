#![deny(clippy::all)]

use std::sync::Arc;
use farmfe_core::{
    config::Config,
    context::CompilationContext,
    plugin::Plugin,
    error::Result,
};
use farmfe_macro_plugin::farm_plugin;
use serde::{Deserialize};

#[farm_plugin]
pub struct FarmBundleSizeAnalyzerPlugin {
    config: AnalyzerConfig,
}

#[derive(Debug, Deserialize)]
struct AnalyzerConfig {
    #[serde(default = "default_warning_threshold")]
    warning_threshold: usize,
    #[serde(default = "default_show_suggestions")]
    show_suggestions: bool,
    #[serde(default = "default_generate_report")]
    generate_report: bool,
}

fn default_warning_threshold() -> usize { 1024 * 1024 }
fn default_show_suggestions() -> bool { true }
fn default_generate_report() -> bool { false }

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            warning_threshold: default_warning_threshold(),
            show_suggestions: default_show_suggestions(),
            generate_report: default_generate_report(),
        }
    }
}

impl FarmBundleSizeAnalyzerPlugin {
    pub fn new(_config: &Config, options: String) -> Self {
        let analyzer_config: AnalyzerConfig = if options.is_empty() {
            AnalyzerConfig::default()
        } else {
            serde_json::from_str(&options).unwrap_or_else(|_| AnalyzerConfig::default())
        };
        
        Self {
            config: analyzer_config,
        }
    }

    fn analyze_and_display_bundle_size(&self, context: &Arc<CompilationContext>) -> Result<Option<()>> {
        let resources_map = context.resources_map.lock();

        if resources_map.is_empty() {
            println!("No resources found in the bundle");
            return Ok(Some(()));
        }

        println!("\nFarm Bundle Size Analysis");
        println!("============================================");

        let mut total_size = 0;
        let mut files_info = Vec::new();

        for (name, resource) in resources_map.iter() {
            let size = resource.bytes.len();
            total_size += size;
            files_info.push((name.clone(), size));
        }

        // Sort by file size
        files_info.sort_by(|a, b| b.1.cmp(&a.1));

        for (name, size) in &files_info {
            let size_str = format_size(*size);
            let percentage = (*size as f64 / total_size as f64) * 100.0;
            
            print!("{}: {} ({:.1}%)", name, size_str, percentage);
            
            if *size > self.config.warning_threshold {
                print!(" [LARGE FILE]");
            }
            println!();
        }

        println!("\nSummary");
        println!("--------------------------------------------");
        println!("Total files: {}", files_info.len());
        println!("Total size: {}", format_size(total_size));
        println!("Estimated gzipped: ~{}", format_size((total_size as f64 * 0.35) as usize));

        let large_files: Vec<_> = files_info.iter()
            .filter(|(_, size)| *size > self.config.warning_threshold)
            .collect();

        if !large_files.is_empty() && self.config.show_suggestions {
            println!("\nOptimization Suggestions");
            println!("--------------------------------------------");
            println!("WARNING: {} large files detected (> {})", 
                large_files.len(), 
                format_size(self.config.warning_threshold)
            );
            for (name, size) in large_files {
                println!("   - {}: {}", name, format_size(*size));
            }
            println!("   Consider code splitting or lazy loading for these files.");
        }

        Ok(Some(()))
    }
}

impl Plugin for FarmBundleSizeAnalyzerPlugin {
    fn name(&self) -> &str {
        "farm-bundle-size-analyzer"
    }

    fn generate_end(&self, context: &Arc<CompilationContext>) -> Result<Option<()>> {
        self.analyze_and_display_bundle_size(context)
    }
}

fn format_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as usize, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use farmfe_core::{
        config::Config,
        context::CompilationContext,
        resource::Resource,
    };

    // 创建测试用的 CompilationContext
    fn create_test_context() -> Arc<CompilationContext> {
        let mut config = Config::default();
        config.root = std::env::temp_dir().join("farm_test").to_string_lossy().to_string();
        
        Arc::new(CompilationContext {
            config: Box::new(config),
            resources_map: Box::new(farmfe_core::parking_lot::Mutex::new(HashMap::new())),
            ..Default::default()
        })
    }

    // 创建测试用的 Resource
    fn create_test_resource(size: usize) -> Resource {
        Resource {
            bytes: vec![0u8; size],
            ..Default::default()
        }
    }

    #[test]
    fn test_plugin_creation_with_default_config() {
        let config = Config::default();
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
        
        assert_eq!(plugin.name(), "farm-bundle-size-analyzer");
        assert_eq!(plugin.config.warning_threshold, 1024 * 1024); // 1MB
        assert!(plugin.config.show_suggestions);
        assert!(!plugin.config.generate_report);
    }

    #[test]
    fn test_plugin_creation_with_custom_config() {
        let config = Config::default();
        let custom_options = r#"{
            "warning_threshold": 2097152,
            "show_suggestions": false,
            "generate_report": true
        }"#;
        
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, custom_options.to_string());
        
        assert_eq!(plugin.config.warning_threshold, 2097152); // 2MB
        assert!(!plugin.config.show_suggestions);
        assert!(plugin.config.generate_report);
    }

    #[test]
    fn test_plugin_creation_with_invalid_config() {
        let config = Config::default();
        let invalid_options = r#"{"invalid": "json"}"#;
        
        // 应该使用默认配置
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, invalid_options.to_string());
        
        assert_eq!(plugin.config.warning_threshold, 1024 * 1024); // 默认值
        assert!(plugin.config.show_suggestions);
        assert!(!plugin.config.generate_report);
    }

    #[test]
    fn test_analyze_empty_bundle() {
        let config = Config::default();
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
        let context = create_test_context();
        
        let result = plugin.analyze_and_display_bundle_size(&context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(()));
    }

    #[test]
    fn test_analyze_bundle_with_files() {
        let config = Config::default();
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
        let context = create_test_context();
        
        // 添加测试资源
        {
            let mut resources_map = context.resources_map.lock();
            resources_map.insert("main.js".to_string(), create_test_resource(1024));
            resources_map.insert("style.css".to_string(), create_test_resource(512));
        }
        
        let result = plugin.analyze_and_display_bundle_size(&context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(()));
    }

    #[test]
    fn test_analyze_large_files() {
        let config = Config::default();
        let custom_options = r#"{"warning_threshold": 1024}"#; // 1KB 阈值
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, custom_options.to_string());
        let context = create_test_context();
        
        // 添加一个大文件
        {
            let mut resources_map = context.resources_map.lock();
            resources_map.insert("large.js".to_string(), create_test_resource(2048)); // 2KB
            resources_map.insert("small.js".to_string(), create_test_resource(512));  // 512B
        }
        
        let result = plugin.analyze_and_display_bundle_size(&context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(()));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_analyzer_config_defaults() {
        let config = AnalyzerConfig::default();
        
        assert_eq!(config.warning_threshold, 1024 * 1024);
        assert!(config.show_suggestions);
        assert!(!config.generate_report);
    }

    #[test]
    fn test_analyzer_config_deserialization() {
        let json = r#"{
            "warning_threshold": 2097152,
            "show_suggestions": false,
            "generate_report": true
        }"#;
        
        let config: AnalyzerConfig = serde_json::from_str(json).unwrap();
        
        assert_eq!(config.warning_threshold, 2097152);
        assert!(!config.show_suggestions);
        assert!(config.generate_report);
    }

    #[test]
    fn test_analyzer_config_partial_deserialization() {
        let json = r#"{"warning_threshold": 2097152}"#;
        
        let config: AnalyzerConfig = serde_json::from_str(json).unwrap();
        
        assert_eq!(config.warning_threshold, 2097152);
        assert!(config.show_suggestions); // 默认值
        assert!(!config.generate_report); // 默认值
    }

    #[test]
    fn test_plugin_generate_end_hook() {
        let config = Config::default();
        let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
        let context = create_test_context();
        
        // 添加一些测试资源
        {
            let mut resources_map = context.resources_map.lock();
            resources_map.insert("test.js".to_string(), create_test_resource(1024));
        }
        
        let result = plugin.generate_end(&context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(()));
    }
}