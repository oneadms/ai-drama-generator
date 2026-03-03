import { useEffect, useState } from 'react';

interface Progress {
  stage: 'script' | 'voice' | 'video';
  current_episode: number;
  total_episodes: number;
  message: string;
  percentage: number;
}

interface Props {
  taskId: string;
  onComplete?: () => void;
}

export function ProgressDisplay({ taskId, onComplete }: Props) {
  const [progress, setProgress] = useState<Progress | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const eventSource = new EventSource(`http://localhost:3000/api/progress/${taskId}`);

    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setProgress(data.progress);
      
      if (data.progress.percentage >= 100) {
        eventSource.close();
        onComplete?.();
      }
    };

    eventSource.onerror = () => {
      setError('连接断开');
      eventSource.close();
    };

    return () => eventSource.close();
  }, [taskId, onComplete]);

  if (error) return <div style={{ color: 'red' }}>{error}</div>;
  if (!progress) return <div>连接中...</div>;

  const stageNames = { script: '剧本生成', voice: '语音生成', video: '视频生成' };

  return (
    <div style={{ padding: '20px' }}>
      <div>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '10px' }}>
          <span style={{ fontWeight: 'bold' }}>{stageNames[progress.stage]}</span>
          <span style={{ fontSize: '14px', color: '#666' }}>
            第 {progress.current_episode}/{progress.total_episodes} 集
          </span>
        </div>
        
        <div style={{ width: '100%', background: '#eee', height: '8px', borderRadius: '4px' }}>
          <div
            style={{ 
              width: `${progress.percentage}%`, 
              background: '#2563eb', 
              height: '100%',
              borderRadius: '4px',
              transition: 'width 0.3s'
            }}
          />
        </div>
        
        <p style={{ fontSize: '14px', color: '#666', marginTop: '10px' }}>{progress.message}</p>
      </div>
    </div>
  );
}
