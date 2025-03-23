import axios from 'axios';
import { Task, TaskFormData, TaskUpdateData, ApiResponse } from '../types/Task';

const api = axios.create({
    baseURL: '/',
    headers: {
        'Content-Type': 'application/json',
    },
});

export const getTasks = async (): Promise<Task[]> => {
    try {
        const response = await api.get<ApiResponse<Task[]>>('/tasks');
        if (response.data.success && response.data.data) {
            return response.data.data;
        }
        return [];
    } catch (error) {
        console.error('Erro ao buscar tarefas:', error);
        return [];
    }
};

export const getTask = async (id: string): Promise<Task | null> => {
    try {
        const response = await api.get<ApiResponse<Task>>(`/tasks/${id}`);
        if (response.data.success && response.data.data) {
            return response.data.data;
        }
        return null;
    } catch (error) {
        console.error(`Erro ao buscar tarefa ${id}:`, error);
        return null;
    }
};

export const createTask = async (taskData: TaskFormData): Promise<Task | null> => {
    try {
        const response = await api.post<ApiResponse<Task>>('/tasks', taskData);
        if (response.data.success && response.data.data) {
            return response.data.data;
        }
        return null;
    } catch (error) {
        console.error('Erro ao criar tarefa:', error);
        return null;
    }
};

export const updateTask = async (id: string, taskData: TaskUpdateData): Promise<Task | null> => {
    try {
        const response = await api.put<ApiResponse<Task>>(`/tasks/${id}`, taskData);
        if (response.data.success && response.data.data) {
            return response.data.data;
        }
        return null;
    } catch (error) {
        console.error(`Erro ao atualizar tarefa ${id}:`, error);
        return null;
    }
};

export const deleteTask = async (id: string): Promise<boolean> => {
    try {
        const response = await api.delete<ApiResponse<null>>(`/tasks/${id}`);
        return response.data.success;
    } catch (error) {
        console.error(`Erro ao excluir tarefa ${id}:`, error);
        return false;
    }
};

export default api; 