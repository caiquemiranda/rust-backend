import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
    Box,
    Button,
    Card,
    CardContent,
    Chip,
    CircularProgress,
    Dialog,
    DialogActions,
    DialogContent,
    DialogContentText,
    DialogTitle,
    Divider,
    FormControl,
    Grid,
    InputLabel,
    MenuItem,
    Paper,
    Select,
    Typography,
    Alert,
} from '@mui/material';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import { useTaskContext } from '../contexts/TaskContext';
import {
    Task,
    TaskPriority,
    TaskPriorityColors,
    TaskPriorityLabels,
    TaskStatus,
    TaskStatusColors,
} from '../types/Task';

const TaskList: React.FC = () => {
    const { tasks, loading, error, removeTask } = useTaskContext();
    const navigate = useNavigate();
    const [statusFilter, setStatusFilter] = useState<string>('');
    const [priorityFilter, setPriorityFilter] = useState<number>(0);
    const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
    const [taskToDelete, setTaskToDelete] = useState<string | null>(null);

    // Função para formatar data
    const formatDate = (dateString: string) => {
        const date = new Date(dateString);
        return date.toLocaleDateString('pt-BR', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
        });
    };

    // Filtrar tarefas baseado nos filtros selecionados
    const filteredTasks = tasks.filter((task) => {
        if (statusFilter && task.status !== statusFilter) return false;
        if (priorityFilter > 0 && task.priority !== priorityFilter) return false;
        return true;
    });

    // Lidar com edição de tarefa
    const handleEdit = (id: string) => {
        navigate(`/tasks/edit/${id}`);
    };

    // Abrir diálogo de confirmação de exclusão
    const handleDeleteConfirmation = (id: string) => {
        setTaskToDelete(id);
        setDeleteDialogOpen(true);
    };

    // Realizar exclusão após confirmação
    const handleDelete = async () => {
        if (taskToDelete) {
            await removeTask(taskToDelete);
            setDeleteDialogOpen(false);
            setTaskToDelete(null);
        }
    };

    // Cancelar exclusão
    const handleCancelDelete = () => {
        setDeleteDialogOpen(false);
        setTaskToDelete(null);
    };

    // Exibir carregamento
    if (loading) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
                <CircularProgress />
            </Box>
        );
    }

    // Exibir erro
    if (error) {
        return (
            <Alert severity="error" sx={{ mt: 2 }}>
                {error}
            </Alert>
        );
    }

    return (
        <>
            <Typography variant="h4" gutterBottom>
                Minhas Tarefas
            </Typography>

            {/* Filtros */}
            <Paper sx={{ p: 2, mb: 3 }}>
                <Grid container spacing={2}>
                    <Grid item xs={12} sm={6}>
                        <FormControl fullWidth size="small">
                            <InputLabel>Filtrar por Status</InputLabel>
                            <Select
                                value={statusFilter}
                                label="Filtrar por Status"
                                onChange={(e) => setStatusFilter(e.target.value)}
                            >
                                <MenuItem value="">Todos</MenuItem>
                                {Object.values(TaskStatus).map((status) => (
                                    <MenuItem key={status} value={status}>
                                        {status}
                                    </MenuItem>
                                ))}
                            </Select>
                        </FormControl>
                    </Grid>
                    <Grid item xs={12} sm={6}>
                        <FormControl fullWidth size="small">
                            <InputLabel>Filtrar por Prioridade</InputLabel>
                            <Select
                                value={priorityFilter}
                                label="Filtrar por Prioridade"
                                onChange={(e) => setPriorityFilter(Number(e.target.value))}
                            >
                                <MenuItem value={0}>Todas</MenuItem>
                                {Object.entries(TaskPriority)
                                    .filter(([key]) => isNaN(Number(key)))
                                    .map(([_, value]) => (
                                        <MenuItem key={value} value={value}>
                                            {TaskPriorityLabels[value as TaskPriority]}
                                        </MenuItem>
                                    ))}
                            </Select>
                        </FormControl>
                    </Grid>
                </Grid>
            </Paper>

            {/* Lista de tarefas */}
            {filteredTasks.length === 0 ? (
                <Paper sx={{ p: 3, textAlign: 'center' }}>
                    <Typography variant="body1">
                        Nenhuma tarefa encontrada. {statusFilter || priorityFilter ? 'Tente remover os filtros.' : ''}
                    </Typography>
                </Paper>
            ) : (
                <Grid container spacing={2}>
                    {filteredTasks.map((task: Task) => (
                        <Grid item xs={12} key={task.id}>
                            <Card sx={{ mb: 2 }}>
                                <CardContent>
                                    <Box sx={{ display: 'flex', justifyContent: 'space-between', flexWrap: 'wrap' }}>
                                        <Box sx={{ flexGrow: 1 }}>
                                            <Typography variant="h6" component="div" gutterBottom>
                                                {task.title}
                                            </Typography>
                                            <Typography
                                                variant="body2"
                                                color="text.secondary"
                                                sx={{ whiteSpace: 'pre-wrap', mb: 2 }}
                                            >
                                                {task.description}
                                            </Typography>
                                            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1, mb: 2 }}>
                                                <Chip
                                                    label={task.status}
                                                    size="small"
                                                    sx={{
                                                        bgcolor: TaskStatusColors[task.status] || '#888',
                                                        color: 'white',
                                                    }}
                                                />
                                                <Chip
                                                    label={TaskPriorityLabels[task.priority as TaskPriority]}
                                                    size="small"
                                                    sx={{
                                                        bgcolor: TaskPriorityColors[task.priority as TaskPriority] || '#888',
                                                        color: 'white',
                                                    }}
                                                />
                                            </Box>
                                            <Divider sx={{ mb: 1 }} />
                                            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
                                                <Typography variant="caption" color="text.secondary">
                                                    Criada em: {formatDate(task.created_at)}
                                                </Typography>
                                                <Typography variant="caption" color="text.secondary">
                                                    Atualizada em: {formatDate(task.updated_at)}
                                                </Typography>
                                            </Box>
                                        </Box>
                                        <Box sx={{ display: 'flex', alignItems: 'flex-start', ml: 2 }}>
                                            <Button
                                                size="small"
                                                startIcon={<EditIcon />}
                                                onClick={() => handleEdit(task.id)}
                                                sx={{ mr: 1 }}
                                            >
                                                Editar
                                            </Button>
                                            <Button
                                                size="small"
                                                color="error"
                                                startIcon={<DeleteIcon />}
                                                onClick={() => handleDeleteConfirmation(task.id)}
                                            >
                                                Excluir
                                            </Button>
                                        </Box>
                                    </Box>
                                </CardContent>
                            </Card>
                        </Grid>
                    ))}
                </Grid>
            )}

            {/* Diálogo de confirmação de exclusão */}
            <Dialog open={deleteDialogOpen} onClose={handleCancelDelete}>
                <DialogTitle>Confirmar Exclusão</DialogTitle>
                <DialogContent>
                    <DialogContentText>
                        Tem certeza de que deseja excluir esta tarefa? Esta ação não pode ser desfeita.
                    </DialogContentText>
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleCancelDelete}>Cancelar</Button>
                    <Button onClick={handleDelete} color="error" autoFocus>
                        Excluir
                    </Button>
                </DialogActions>
            </Dialog>
        </>
    );
};

export default TaskList; 