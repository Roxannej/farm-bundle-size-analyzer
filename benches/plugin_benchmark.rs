//! 基准测试 - 测试插件性能

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use farm_bundle_size_analyzer::FarmBundleSizeAnalyzerPlugin;
use farmfe_core::{
    config::Config,
    context::CompilationContext,
    resource::Resource,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn create_benchmark_context(file_count: usize, avg_file_size: usize) -> Arc<CompilationContext> {
    let mut config = Config::default();
    config.root = std::env::temp_dir().join("farm_benchmark");
    
    let mut resources_map = HashMap::new();
    
    for i in 0..file_count {
        let filename = format!("file_{}.js", i);
        let size = avg_file_size + (i % 1000); // 添加一些变化
        let resource = Resource {
            bytes: vec![0u8; size],
            ..Default::default()
        };
        resources_map.insert(filename, Arc::new(resource));
    }
    
    Arc::new(CompilationContext {
        config: Arc::new(config),
        resources_map: Arc::new(Mutex::new(resources_map)),
        ..Default::default()
    })
}

fn benchmark_plugin_analysis(c: &mut Criterion) {
    let config = Config::default();
    let plugin = FarmBundleSizeAnalyzerPlugin::new(&config, String::new());
    
    c.bench_function("analyze_small_bundle", |b| {
        let context = create_benchmark_context(10, 1024);
        b.iter(|| {
            let _ = plugin.analyze_and_display_bundle_size(black_box(&context));
        });
    });
    
    c.bench_function("analyze_medium_bundle", |b| {
        let context = create_benchmark_context(100, 10 * 1024);
        b.iter(|| {
            let _ = plugin.analyze_and_display_bundle_size(black_box(&context));
        });
    });
    
    c.bench_function("analyze_large_bundle", |b| {
        let context = create_benchmark_context(1000, 100 * 1024);
        b.iter(|| {
            let _ = plugin.analyze_and_display_bundle_size(black_box(&context));
        });
    });
}

fn benchmark_plugin_creation(c: &mut Criterion) {
    let config = Config::default();
    
    c.bench_function("create_plugin_default", |b| {
        b.iter(|| {
            let _ = FarmBundleSizeAnalyzerPlugin::new(black_box(&config), String::new());
        });
    });
    
    c.bench_function("create_plugin_with_config", |b| {
        let config_json = r#"{
            "warning_threshold": 2097152,
            "show_suggestions": true,
            "generate_report": false
        }"#;
        b.iter(|| {
            let _ = FarmBundleSizeAnalyzerPlugin::new(
                black_box(&config), 
                black_box(config_json.to_string())
            );
        });
    });
}

criterion_group!(benches, benchmark_plugin_analysis, benchmark_plugin_creation);
criterion_main!(benches);
