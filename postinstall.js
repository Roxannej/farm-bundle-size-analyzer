const fs = require("fs");
const path = require("path");

console.log("📦 Farm Bundle Size Analyzer plugin installed successfully!");

// 检查当前平台的动态库是否存在
const os = require("os");
const platform = os.platform();
const arch = os.arch();

let platformDir;
if (platform === "darwin") {
  platformDir = arch === "arm64" ? "darwin-arm64" : "darwin-x64";
} else if (platform === "linux") {
  platformDir = "linux-x64-gnu";
} else if (platform === "win32") {
  platformDir = "win32-x64-msvc";
}

if (platformDir) {
  const libPath = path.join(__dirname, platformDir);
  if (fs.existsSync(libPath)) {
    console.log(`✅ Found platform-specific library: ${platformDir}`);
  } else {
    console.warn(`⚠️  Platform library not found: ${platformDir}`);
  }
}
