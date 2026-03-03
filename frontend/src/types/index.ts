export interface DramaConfig {
  projectName: string;
  genre: string;
  episodes: number;
  scriptProvider: string;
  voiceProvider: string;
}

export interface TaskResponse {
  taskId: string;
  status: string;
}

export interface ProgressEvent {
  taskId: string;
  episode: number;
  total: number;
  status: string;
  message: string;
}
