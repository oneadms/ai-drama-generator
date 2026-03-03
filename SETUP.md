# 🎯 项目完整落地 - 配置清单

## ✅ 已完成

项目已完整构建，包含：
- ✅ Rust 核心引擎（异步 + 插件化）
- ✅ 多服务商支持（OpenAI/Claude/Azure/Edge）
- ✅ 断点续传功能
- ✅ 完整文档

## 📋 需要配置的内容

### 1. 环境变量 (.env)

```bash
# 必需（至少配置一个）
export OPENAI_API_KEY="sk-xxx"           # OpenAI API
export ANTHROPIC_API_KEY="sk-ant-xxx"   # Claude API（可选）
export AZURE_SPEECH_KEY="xxx"           # Azure TTS（可选）

# 可选（如需代理）
export HTTP_PROXY="http://127.0.0.1:7897"
export HTTPS_PROXY="http://127.0.0.1:7897"
```

### 2. 配置文件 (config.toml)

修改以下关键参数：
- `project.name` - 短剧名称
- `project.episodes` - 总集数
- `script.provider` - openai 或 claude
- `script.model` - gpt-4 或 gpt-3.5-turbo
- `voice.provider` - edge（免费）或 azure（付费）

### 3. 素材准备

```bash
# 必需
assets/avatar.png          # 数字人头像（正面照）

# 可选
assets/music/bg.mp3        # 背景音乐
assets/backgrounds/bg.jpg  # 背景图片
```

### 4. 依赖安装

```bash
# Python 依赖
pip3 install edge-tts

# SadTalker（视频生成）
git clone https://github.com/OpenTalker/SadTalker.git
cd SadTalker && pip3 install -r requirements.txt
```

## 🚀 运行流程

```bash
# 1. 加载环境变量
source .env

# 2. 运行生成器
./target/release/ai-drama-generator

# 3. 查看输出
ls output/
```

## 💰 成本预估

**免费方案**：$2-5 / 100集
**付费方案**：$25-40 / 100集

## 📊 预期收益

- 10万播放 = ¥200-800
- 100万播放 = ¥2000-8000
- ROI: 2.5-10倍

---

**项目位置**：`/Users/oneadm/.openclaw/workspace/ai-drama-generator`
