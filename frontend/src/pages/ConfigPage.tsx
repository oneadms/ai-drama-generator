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
    <div style={{ padding: '20px' }}>
      <h1>AI 短剧生成器</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <label>项目名称:</label>
          <input
            value={config.projectName}
            onChange={(e) => setConfig({ ...config, projectName: e.target.value })}
          />
        </div>
        <div>
          <label>集数:</label>
          <input
            type="number"
            value={config.episodes}
            onChange={(e) => setConfig({ ...config, episodes: +e.target.value })}
          />
        </div>
        <button type="submit">开始生成</button>
      </form>
    </div>
  );
}
