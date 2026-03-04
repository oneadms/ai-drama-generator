import { useState } from 'react';
import { dramaApi } from '../api/client';
import type { DramaConfig } from '../types';
import ProgressPage from './ProgressPage';

export default function ConfigPage() {
  const [config, setConfig] = useState<DramaConfig>({
    projectName: '',
    genre: 'romance',
    episodes: 10,
    scriptProvider: 'claude',
    voiceProvider: 'edge',
  });
  const [apiKeys, setApiKeys] = useState({
    scriptApiKey: '',
    voiceApiKey: '',
  });
  const [taskId, setTaskId] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const res = await dramaApi.createTask(config);
    setTaskId(res.data.taskId);
  };

  if (taskId) {
    return <ProgressPage taskId={taskId} />;
  }

  return (
    <div style={{ padding: '20px', maxWidth: '600px' }}>
      <h1>AI 短剧生成器</h1>
      <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
        
        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>项目名称:</label>
          <input
            style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
            value={config.projectName}
            onChange={(e) => setConfig({ ...config, projectName: e.target.value })}
            required
          />
        </div>

        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>类型:</label>
          <select
            style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
            value={config.genre}
            onChange={(e) => setConfig({ ...config, genre: e.target.value })}
          >
            <option value="romance">言情</option>
            <option value="urban">都市</option>
            <option value="fantasy">玄幻</option>
            <option value="suspense">悬疑</option>
          </select>
        </div>

        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>集数:</label>
          <input
            style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
            type="number"
            min="1"
            max="100"
            value={config.episodes}
            onChange={(e) => setConfig({ ...config, episodes: +e.target.value })}
          />
        </div>

        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>剧本生成:</label>
          <select
            style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
            value={config.scriptProvider}
            onChange={(e) => setConfig({ ...config, scriptProvider: e.target.value })}
          >
            <option value="claude">Claude</option>
            <option value="openai">OpenAI</option>
          </select>
        </div>

        {config.scriptProvider !== 'edge' && (
          <div>
            <label style={{ display: 'block', marginBottom: '5px' }}>API Key:</label>
            <input
              style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
              type="password"
              value={apiKeys.scriptApiKey}
              onChange={(e) => setApiKeys({ ...apiKeys, scriptApiKey: e.target.value })}
              placeholder="输入 API Key"
            />
          </div>
        )}

        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>配音生成:</label>
          <select
            style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
            value={config.voiceProvider}
            onChange={(e) => setConfig({ ...config, voiceProvider: e.target.value })}
          >
            <option value="edge">Edge TTS (免费)</option>
            <option value="azure">Azure TTS</option>
          </select>
        </div>

        <button 
          type="submit"
          style={{ 
            padding: '10px 20px', 
            background: '#2563eb', 
            color: 'white', 
            border: 'none', 
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '16px'
          }}
        >
          开始生成
        </button>
      </form>
    </div>
  );
}
