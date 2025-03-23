import React from 'react';
import { Link as RouterLink, useLocation } from 'react-router-dom';
import {
    AppBar,
    Box,
    Button,
    Container,
    IconButton,
    Toolbar,
    Typography,
    useTheme,
} from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import HomeIcon from '@mui/icons-material/Home';
import ListAltIcon from '@mui/icons-material/ListAlt';

const Header: React.FC = () => {
    const theme = useTheme();
    const location = useLocation();
    const isTasksPage = location.pathname === '/tasks';

    return (
        <AppBar position="static">
            <Container maxWidth="lg">
                <Toolbar disableGutters>
                    <Box sx={{ display: 'flex', alignItems: 'center', mr: 2 }}>
                        <ListAltIcon sx={{ mr: 1 }} />
                        <Typography
                            variant="h6"
                            noWrap
                            component={RouterLink}
                            to="/"
                            sx={{
                                fontWeight: 700,
                                letterSpacing: '.1rem',
                                color: 'inherit',
                                textDecoration: 'none',
                            }}
                        >
                            GERENCIADOR DE TAREFAS
                        </Typography>
                    </Box>

                    <Box sx={{ flexGrow: 1 }} />

                    <Box sx={{ display: 'flex', alignItems: 'center' }}>
                        <IconButton
                            component={RouterLink}
                            to="/tasks"
                            color="inherit"
                            sx={{
                                mr: 1,
                                bgcolor: location.pathname === '/tasks' ? 'rgba(255, 255, 255, 0.15)' : 'transparent'
                            }}
                        >
                            <HomeIcon />
                        </IconButton>

                        {isTasksPage && (
                            <Button
                                variant="contained"
                                component={RouterLink}
                                to="/tasks/new"
                                startIcon={<AddIcon />}
                                sx={{
                                    bgcolor: theme.palette.secondary.main,
                                    '&:hover': {
                                        bgcolor: theme.palette.secondary.dark,
                                    },
                                }}
                            >
                                Nova Tarefa
                            </Button>
                        )}
                    </Box>
                </Toolbar>
            </Container>
        </AppBar>
    );
};

export default Header; 