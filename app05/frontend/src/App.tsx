import React from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { Box, Container } from '@mui/material';
import TaskList from './components/TaskList';
import TaskForm from './components/TaskForm';
import Header from './components/Header';
import { TaskProvider } from './contexts/TaskContext';

const App: React.FC = () => {
    return (
        <TaskProvider>
            <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
                <Header />
                <Container component="main" sx={{ flexGrow: 1, py: 3 }}>
                    <Routes>
                        <Route path="/" element={<Navigate to="/tasks" replace />} />
                        <Route path="/tasks" element={<TaskList />} />
                        <Route path="/tasks/new" element={<TaskForm />} />
                        <Route path="/tasks/edit/:id" element={<TaskForm />} />
                    </Routes>
                </Container>
                <Box
                    component="footer"
                    sx={{
                        py: 2,
                        px: 2,
                        mt: 'auto',
                        backgroundColor: (theme) => theme.palette.grey[200],
                        textAlign: 'center',
                    }}
                >
                    Gerenciador de Tarefas com React e Rust © {new Date().getFullYear()}
                </Box>
            </Box>
        </TaskProvider>
    );
};

export default App; 