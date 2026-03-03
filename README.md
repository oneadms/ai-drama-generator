# AI 短剧自动化生成系统

高度配置化的 Rust 短剧生成工具，支持全流程自动化。

## ✨ 特性

- 🎬 全流程自动化：剧本 → 配音 → 视频
- 🔧 高度配置化：TOML 配置所有参数
- 🔌 插件化架构：轻松扩展服务提供商
- ⚡ 异步并发：高效批量处理
- 💾 断点续传：自动保存进度
- 💰 成本优化：支持免费和付费方案

## 🚀 快速开始

```bash
# 1. 安装依赖
./install.sh

# 2. 配置
cp .env.example .env
# 编辑 .env 填入 API Key

# 3. 准备头像
cp your_avatar.png assets/avatar.png

# 4. 运行
source .env
./target/release/ai-drama-generator
```

详见 [快速开始](docs/QUICKSTART.md)

## 📖 文档

- [配置指南](docs/CONFIG.md)
- [成本对比](docs/COST.md)
- [故障排查](docs/TROUBLESHOOTING.md)

## 🏗️ 架构

```
Pipeline
  ├── ScriptGenerator (OpenAI/Claude)
  ├── VoiceGenerator (Azure/Edge TTS)
  └── VideoGenerator (SadTalker)
```

## 💡 扩展

实现对应 trait 即可添加新服务商。
