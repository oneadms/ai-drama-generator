import { ProgressDisplay } from '../components/ProgressDisplay';

export default function ProgressPage({ taskId }: { taskId: string }) {
  return (
    <div style={{ padding: '20px' }}>
      <h2>生成进度</h2>
      <p>任务 ID: {taskId}</p>
      <ProgressDisplay taskId={taskId} onComplete={() => alert('完成!')} />
    </div>
  );
}
