import axios from 'axios';
import type { DramaConfig, TaskResponse } from '../types';

const api = axios.create({
  baseURL: 'http://localhost:3000/api',
});

export const dramaApi = {
  createTask: (config: DramaConfig) =>
    api.post<TaskResponse>('/drama/create', config),
  
  getTask: (id: string) =>
    api.get<TaskResponse>(`/drama/${id}`),
};
