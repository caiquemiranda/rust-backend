import React, { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import {
    Alert,
    Box,
    Button,
    Card,
    CardContent,
    CircularProgress,
    FormControl,
    FormHelperText,
    Grid,
    InputLabel,
    MenuItem,
    Select,
    TextField,
    Typography,
} from '@mui/material';
import { useTaskContext } from '../contexts/TaskContext';
import { getTask } from '../services/api';
import {
    TaskFormData,
    TaskPriority,
    TaskPriorityLabels,
    TaskStatus,
} from '../types/Task';

// Estado inicial do formulário
const initialFormState: TaskFormData = {
    title: '',
    description: '',
    status: TaskStatus.TODO,
    priority: TaskPriority.MEDIUM,
};

// Estado inicial dos erros do formulário
const initialErrorState = {
    title: '',
    description: '',
    status: '',
    priority: '',
};

const TaskForm: React.FC = () => {
    const { id } = useParams<{ id: string }>();
    const navigate = useNavigate();
    const { addTask, editTask } = useTaskContext();
    const [formData, setFormData] = useState<TaskFormData>(initialFormState);
    const [errors, setErrors] = useState(initialErrorState);
    const [loading, setLoading] = useState(false);
    const [fetchingTask, setFetchingTask] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [successMessage, setSuccessMessage] = useState<string | null>(null);

    const isEditMode = Boolean(id);

    // Carregar tarefa existente se estiver em modo de edição
    useEffect(() => {
        if (isEditMode && id) {
            const fetchTaskData = async () => {
                setFetchingTask(true);
                setError(null);
                try {
                    const taskData = await getTask(id);
                    if (taskData) {
                        setFormData({
                            title: taskData.title,
                            description: taskData.description,
                            status: taskData.status,
                            priority: taskData.priority,
                        });
                    } else {
                        setError('Tarefa não encontrada');
                        setTimeout(() => {
                            navigate('/tasks');
                        }, 2000);
                    }
                } catch (err) {
                    setError('Erro ao carregar dados da tarefa');
                } finally {
                    setFetchingTask(false);
                }
            };

            fetchTaskData();
        }
    }, [id, isEditMode, navigate]);

    // Atualizar valor do campo do formulário
    const handleChange = (
        e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | { name?: string; value: unknown }>
    ) => {
        const { name, value } = e.target as { name: string; value: unknown };
        setFormData((prev) => ({
            ...prev,
            [name]: value,
        }));

        // Limpar erro do campo quando o usuário começa a digitar
        if (errors[name as keyof typeof errors]) {
            setErrors((prev) => ({
                ...prev,
                [name]: '',
            }));
        }
    };

    // Validar formulário
    const validateForm = (): boolean => {
        let valid = true;
        const newErrors = { ...initialErrorState };

        if (!formData.title.trim()) {
            newErrors.title = 'O título é obrigatório';
            valid = false;
        } else if (formData.title.length > 100) {
            newErrors.title = 'O título não pode exceder 100 caracteres';
            valid = false;
        }

        if (!formData.description.trim()) {
            newErrors.description = 'A descrição é obrigatória';
            valid = false;
        }

        if (!formData.status) {
            newErrors.status = 'O status é obrigatório';
            valid = false;
        }

        if (!formData.priority) {
            newErrors.priority = 'A prioridade é obrigatória';
            valid = false;
        }

        setErrors(newErrors);
        return valid;
    };

    // Enviar formulário
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setSuccessMessage(null);

        if (!validateForm()) {
            return;
        }

        setLoading(true);
        try {
            if (isEditMode && id) {
                const result = await editTask(id, formData);
                if (result) {
                    setSuccessMessage('Tarefa atualizada com sucesso');
                    setTimeout(() => {
                        navigate('/tasks');
                    }, 1500);
                } else {
                    setError('Falha ao atualizar tarefa. Verifique os dados e tente novamente.');
                }
            } else {
                const result = await addTask(formData);
                if (result) {
                    setSuccessMessage('Tarefa criada com sucesso');
                    setTimeout(() => {
                        navigate('/tasks');
                    }, 1500);
                } else {
                    setError('Falha ao criar tarefa. Verifique os dados e tente novamente.');
                }
            }
        } catch (err) {
            setError('Erro ao processar sua solicitação. Tente novamente mais tarde.');
        } finally {
            setLoading(false);
        }
    };

    // Cancelar e voltar para a lista
    const handleCancel = () => {
        navigate('/tasks');
    };

    // Exibir carregamento
    if (fetchingTask) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
                <CircularProgress />
            </Box>
        );
    }

    return (
        <>
            <Typography variant="h4" gutterBottom>
                {isEditMode ? 'Editar Tarefa' : 'Nova Tarefa'}
            </Typography>
            <Card>
                <CardContent>
                    {error && (
                        <Alert severity="error" sx={{ mb: 2 }}>
                            {error}
                        </Alert>
                    )}
                    {successMessage && (
                        <Alert severity="success" sx={{ mb: 2 }}>
                            {successMessage}
                        </Alert>
                    )}
                    <Box component="form" onSubmit={handleSubmit} noValidate>
                        <Grid container spacing={2}>
                            <Grid item xs={12}>
                                <TextField
                                    required
                                    fullWidth
                                    id="title"
                                    label="Título"
                                    name="title"
                                    value={formData.title}
                                    onChange={handleChange}
                                    error={!!errors.title}
                                    helperText={errors.title}
                                    disabled={loading}
                                />
                            </Grid>
                            <Grid item xs={12}>
                                <TextField
                                    required
                                    fullWidth
                                    id="description"
                                    label="Descrição"
                                    name="description"
                                    multiline
                                    rows={4}
                                    value={formData.description}
                                    onChange={handleChange}
                                    error={!!errors.description}
                                    helperText={errors.description}
                                    disabled={loading}
                                />
                            </Grid>
                            <Grid item xs={12} sm={6}>
                                <FormControl fullWidth error={!!errors.status} disabled={loading}>
                                    <InputLabel id="status-label">Status</InputLabel>
                                    <Select
                                        labelId="status-label"
                                        id="status"
                                        name="status"
                                        value={formData.status}
                                        label="Status"
                                        onChange={handleChange as any}
                                    >
                                        {Object.values(TaskStatus).map((status) => (
                                            <MenuItem key={status} value={status}>
                                                {status}
                                            </MenuItem>
                                        ))}
                                    </Select>
                                    {errors.status && <FormHelperText>{errors.status}</FormHelperText>}
                                </FormControl>
                            </Grid>
                            <Grid item xs={12} sm={6}>
                                <FormControl fullWidth error={!!errors.priority} disabled={loading}>
                                    <InputLabel id="priority-label">Prioridade</InputLabel>
                                    <Select
                                        labelId="priority-label"
                                        id="priority"
                                        name="priority"
                                        value={formData.priority}
                                        label="Prioridade"
                                        onChange={handleChange as any}
                                    >
                                        {Object.entries(TaskPriority)
                                            .filter(([key]) => isNaN(Number(key)))
                                            .map(([_, value]) => (
                                                <MenuItem key={value} value={value}>
                                                    {TaskPriorityLabels[value as TaskPriority]}
                                                </MenuItem>
                                            ))}
                                    </Select>
                                    {errors.priority && <FormHelperText>{errors.priority}</FormHelperText>}
                                </FormControl>
                            </Grid>
                            <Grid item xs={12} sx={{ mt: 2, display: 'flex', justifyContent: 'space-between' }}>
                                <Button
                                    variant="outlined"
                                    color="inherit"
                                    onClick={handleCancel}
                                    disabled={loading}
                                >
                                    Cancelar
                                </Button>
                                <Button
                                    type="submit"
                                    variant="contained"
                                    color="primary"
                                    disabled={loading}
                                    sx={{ minWidth: '120px' }}
                                >
                                    {loading ? <CircularProgress size={24} /> : isEditMode ? 'Atualizar' : 'Criar'}
                                </Button>
                            </Grid>
                        </Grid>
                    </Box>
                </CardContent>
            </Card>
        </>
    );
};

export default TaskForm; 