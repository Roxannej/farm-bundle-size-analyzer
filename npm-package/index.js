const path = require("path");
const os = require("os");

// 根据操作系统和架构选择正确的动态库文件
function getLibraryPath() {
  const platform = os.platform();
  const arch = os.arch();

  let platformDir;
  let libName;

  if (platform === "darwin") {
    if (arch === "arm64") {
      platformDir = "darwin-arm64";
    } else {
      platformDir = "darwin-x64";
    }
    libName = "libfarm_bundle_size_analyzer.dylib";
  } else if (platform === "linux") {
    platformDir = "linux-x64-gnu";
    libName = "libfarm_bundle_size_analyzer.so";
  } else if (platform === "win32") {
    platformDir = "win32-x64-msvc";
    libName = "farm_bundle_size_analyzer.dll";
  } else {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }

  return path.join(__dirname, platformDir, libName);
}

// 支持配置的导出
function withConfig(options = {}) {
  return {
    name: "farm-bundle-size-analyzer",
    path: getLibraryPath(),
    options: {
      warning_threshold: 1024 * 1024, // 1MB
      show_suggestions: true,
      generate_report: false,
      ...options,
    },
  };
}

// Farm.js 插件的标准导出格式 - 导出字符串路径
module.exports = getLibraryPath();
module.exports.withConfig = withConfig;
