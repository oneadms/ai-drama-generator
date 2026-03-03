# 配置指南

## 必需配置项

### 1. API Keys

创建 `.env` 文件：
```bash
cp .env.example .env
```

根据选择的服务商配置：

**方案A：免费方案**
```bash
export OPENAI_API_KEY="sk-xxx"  # 或用 GPT-3.5
```

**方案B：付费方案**
```bash
export OPENAI_API_KEY="sk-xxx"
export AZURE_SPEECH_KEY="xxx"
```

### 2. 配置文件

编辑 `config.toml`：

```toml
[project]
name = "你的短剧名"
episodes = 100  # 总集数
duration_per_episode = 90  # 每集秒数

[script]
provider = "openai"  # 或 "claude"
model = "gpt-4"      # 或 "gpt-3.5-turbo"

[voice]
provider = "edge"    # 免费方案用 edge
voice_name = "zh-CN-XiaoxiaoNeural"

[video]
provider = "sadtalker"
avatar_image = "assets/avatar.png"  # 准备一张头像
```

### 3. 准备素材

```bash
# 头像图片（必需）
cp your_avatar.png assets/avatar.png

# 背景音乐（可选）
cp music.mp3 assets/music/

# 背景图片（可选）
cp bg.jpg assets/backgrounds/
```
