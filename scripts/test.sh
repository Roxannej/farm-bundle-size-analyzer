#!/bin/bash

# Farm Bundle Size Analyzer 测试脚本

set -e

echo "🧪 开始运行 Farm Bundle Size Analyzer 测试..."

# 检查 Rust 是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 未找到 cargo 命令，请先安装 Rust"
    exit 1
fi

# 运行单元测试
echo "📋 运行单元测试..."
cargo test

# 运行集成测试
echo "🔗 运行集成测试..."
cargo test --test integration_tests

# 运行基准测试（可选）
if [ "$1" = "--bench" ]; then
    echo "⚡ 运行基准测试..."
    cargo bench
fi

# 检查代码覆盖率（如果安装了 cargo-tarpaulin）
if command -v cargo-tarpaulin &> /dev/null; then
    echo "📊 生成代码覆盖率报告..."
    cargo tarpaulin --out Html
    echo "✅ 覆盖率报告已生成: tarpaulin-report.html"
fi

# 运行 clippy 检查
echo "🔍 运行 clippy 检查..."
cargo clippy -- -D warnings

# 运行格式化检查
echo "🎨 检查代码格式..."
cargo fmt -- --check

echo "✅ 所有测试通过！"

