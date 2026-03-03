# 快速开始

## 安装依赖

```bash
chmod +x install.sh
./install.sh
```

## 配置

1. 复制环境变量：
```bash
cp .env.example .env
# 编辑 .env 填入你的 API Key
```

2. 准备头像：
```bash
cp your_avatar.png assets/avatar.png
```

3. 修改配置：
```bash
# 编辑 config.toml
nano config.toml
```

## 运行

```bash
# 加载环境变量
source .env

# 运行生成器
./target/release/ai-drama-generator
```

## 断点续传

程序会自动保存进度到 `progress.json`，中断后重新运行会跳过已完成的集数。

## 输出

生成的文件在 `output/` 目录：
- `ep1.mp3` - 配音
- `ep1.mp4` - 视频
