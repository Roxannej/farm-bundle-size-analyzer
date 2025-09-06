import "./style.css";

console.log("Hello from Farm.js!");

// 添加一些代码增加文件大小
const data = {
  message: "This is a test application",
  features: ["bundling", "tree-shaking", "hot-reload"],
  config: {
    dev: true,
    production: false,
  },
};

function init() {
  console.log("Initializing app with:", data);
}

init();
