#!/bin/bash
set -e

echo "=== AI 短剧生成器安装脚本 ==="

# 1. 安装 Edge TTS (免费方案)
echo "安装 Edge TTS..."
pip3 install edge-tts

# 2. 克隆 SadTalker
if [ ! -d "SadTalker" ]; then
    echo "克隆 SadTalker..."
    git clone https://github.com/OpenTalker/SadTalker.git
    cd SadTalker
    pip3 install -r requirements.txt
    cd ..
fi

# 3. 创建必要目录
mkdir -p assets/music assets/sfx assets/backgrounds output

echo "✅ 安装完成！"
echo ""
echo "下一步："
echo "1. 配置 config.toml"
echo "2. 准备头像图片放到 assets/avatar.png"
echo "3. 运行: ./target/release/ai-drama-generator"
