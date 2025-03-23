import React, { createContext, useState, useEffect, useContext, useCallback } from 'react';
import { Task, TaskFormData, TaskUpdateData } from '../types/Task';
import { getTasks, createTask, updateTask, deleteTask } from '../services/api';

interface TaskContextData {
    tasks: Task[];
    loading: boolean;
    error: string | null;
    fetchTasks: () => Promise<void>;
    addTask: (taskData: TaskFormData) => Promise<Task | null>;
    editTask: (id: string, taskData: TaskUpdateData) => Promise<Task | null>;
    removeTask: (id: string) => Promise<boolean>;
}

const TaskContext = createContext<TaskContextData>({} as TaskContextData);

export const TaskProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [tasks, setTasks] = useState<Task[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const fetchTasks = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);
            const data = await getTasks();
            setTasks(data);
        } catch (err) {
            setError('Falha ao carregar tarefas. Tente novamente mais tarde.');
            console.error('Erro ao buscar tarefas:', err);
        } finally {
            setLoading(false);
        }
    }, []);

    const addTask = useCallback(async (taskData: TaskFormData): Promise<Task | null> => {
        try {
            const newTask = await createTask(taskData);
            if (newTask) {
                setTasks((prevTasks) => [newTask, ...prevTasks]);
                return newTask;
            }
            return null;
        } catch (err) {
            setError('Falha ao criar tarefa. Tente novamente.');
            console.error('Erro ao adicionar tarefa:', err);
            return null;
        }
    }, []);

    const editTask = useCallback(async (id: string, taskData: TaskUpdateData): Promise<Task | null> => {
        try {
            const updatedTask = await updateTask(id, taskData);
            if (updatedTask) {
                setTasks((prevTasks) =>
                    prevTasks.map((task) => (task.id === id ? updatedTask : task))
                );
                return updatedTask;
            }
            return null;
        } catch (err) {
            setError('Falha ao atualizar tarefa. Tente novamente.');
            console.error('Erro ao editar tarefa:', err);
            return null;
        }
    }, []);

    const removeTask = useCallback(async (id: string): Promise<boolean> => {
        try {
            const success = await deleteTask(id);
            if (success) {
                setTasks((prevTasks) => prevTasks.filter((task) => task.id !== id));
                return true;
            }
            return false;
        } catch (err) {
            setError('Falha ao remover tarefa. Tente novamente.');
            console.error('Erro ao remover tarefa:', err);
            return false;
        }
    }, []);

    useEffect(() => {
        fetchTasks();
    }, [fetchTasks]);

    return (
        <TaskContext.Provider
            value={{
                tasks,
                loading,
                error,
                fetchTasks,
                addTask,
                editTask,
                removeTask,
            }}
        >
            {children}
        </TaskContext.Provider>
    );
};

export const useTaskContext = (): TaskContextData => {
    const context = useContext(TaskContext);
    if (!context) {
        throw new Error('useTaskContext must be used within a TaskProvider');
    }
    return context;
};

export default TaskContext; 