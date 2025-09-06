# Farm Bundle Size Analyzer

A powerful Farm.js plugin to analyze bundle sizes and provide optimization insights.

## 🚀 Features

- **Bundle Analysis**: Analyze JavaScript, CSS, and other asset files
- **Size Warnings**: Configurable warnings for large files
- **Optimization Suggestions**: Get actionable optimization recommendations
- **Multiple Output Formats**: Console output, reports, and charts
- **Performance Monitoring**: Track bundle size trends over time

## 📦 Installation

### NPM
```bash
npm install farm-bundle-size-analyzer
```

### Yarn
```bash
yarn add farm-bundle-size-analyzer
```

## 🛠️ Usage

### Basic Usage

```javascript
// farm.config.js
import bundleAnalyzer from 'farm-bundle-size-analyzer';

export default {
  plugins: [
    bundleAnalyzer.withConfig({
      warning_threshold: 1024 * 1024, // 1MB
      show_suggestions: true,
      generate_report: true
    })
  ]
};
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `warning_threshold` | `number` | `1048576` | Warning threshold in bytes |
| `show_suggestions` | `boolean` | `true` | Show optimization suggestions |
| `generate_report` | `boolean` | `false` | Generate detailed report |

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

## 📊 Example Output

```
Farm Bundle Size Analysis
============================================
main.js: 245.2 KB (60.0%)
styles.css: 82.1 KB (20.0%)
vendor.js: 81.7 KB (20.0%)

Summary
--------------------------------------------
Total files: 3
Total size: 409.0 KB
Estimated gzipped: ~102.3 KB

⚠️  Warning: main.js exceeds threshold (245.2 KB > 1.0 MB)
💡 Suggestion: Consider code splitting for main.js
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/Roxannej/farm-bundle-size-analyzer)
- [NPM Package](https://www.npmjs.com/package/farm-bundle-size-analyzer)
- [Farm.js Documentation](https://farm-fe.github.io/)
