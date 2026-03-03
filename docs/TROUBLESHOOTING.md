# 故障排查

## 常见问题

### 1. Edge TTS 未安装
```bash
pip3 install edge-tts
```

### 2. SadTalker 报错
确保已安装依赖：
```bash
cd SadTalker
pip3 install -r requirements.txt
```

### 3. API Key 无效
检查 .env 文件是否正确加载：
```bash
source .env
echo $OPENAI_API_KEY
```

### 4. 代理问题
如需代理，在 .env 添加：
```bash
export HTTP_PROXY="http://127.0.0.1:7897"
export HTTPS_PROXY="http://127.0.0.1:7897"
```

### 5. 进度丢失
删除 progress.json 重新开始：
```bash
rm progress.json
```
