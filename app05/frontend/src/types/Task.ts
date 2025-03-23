export interface Task {
    id: string;
    title: string;
    description: string;
    status: string;
    priority: number;
    created_at: string;
    updated_at: string;
}

export type TaskFormData = Omit<Task, 'id' | 'created_at' | 'updated_at'>;

export type TaskUpdateData = Partial<TaskFormData>;

export interface ApiResponse<T> {
    success: boolean;
    message: string;
    data?: T;
}

export enum TaskStatus {
    TODO = 'Pendente',
    IN_PROGRESS = 'Em Andamento',
    DONE = 'Concluída',
    CANCELLED = 'Cancelada',
}

export enum TaskPriority {
    LOW = 1,
    MEDIUM = 2,
    HIGH = 3,
    URGENT = 4,
}

export const TaskPriorityLabels: Record<TaskPriority, string> = {
    [TaskPriority.LOW]: 'Baixa',
    [TaskPriority.MEDIUM]: 'Média',
    [TaskPriority.HIGH]: 'Alta',
    [TaskPriority.URGENT]: 'Urgente',
};

export const TaskPriorityColors: Record<TaskPriority, string> = {
    [TaskPriority.LOW]: '#58b09c',
    [TaskPriority.MEDIUM]: '#f9c74f',
    [TaskPriority.HIGH]: '#f8961e',
    [TaskPriority.URGENT]: '#f25c54',
};

export const TaskStatusColors: Record<string, string> = {
    [TaskStatus.TODO]: '#1976d2',
    [TaskStatus.IN_PROGRESS]: '#9c27b0',
    [TaskStatus.DONE]: '#388e3c',
    [TaskStatus.CANCELLED]: '#757575',
}; 