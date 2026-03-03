import { useEffect, useState } from 'react';

interface ProgressData {
  taskId: string;
  progress: number;
  status: string;
}

export default function ProgressPage({ taskId }: { taskId: string }) {
  const [progress, setProgress] = useState<ProgressData | null>(null);

  useEffect(() => {
    const eventSource = new EventSource(`http://localhost:3000/api/progress/${taskId}`);
    
    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setProgress(data);
      
      if (data.progress >= 100) {
        eventSource.close();
      }
    };
    
    return () => eventSource.close();
  }, [taskId]);

  return (
    <div style={{ padding: '20px' }}>
      <h2>生成进度</h2>
      {progress && (
        <div>
          <p>任务 ID: {progress.taskId}</p>
          <p>进度: {progress.progress}%</p>
          <p>状态: {progress.status}</p>
          <div style={{ width: '100%', background: '#eee', height: '20px' }}>
            <div style={{ 
              width: `${progress.progress}%`, 
              background: '#4CAF50', 
              height: '100%' 
            }} />
          </div>
        </div>
      )}
    </div>
  );
}
