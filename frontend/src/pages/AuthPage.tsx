import React from 'react';
import { Container, Typography, Grid, Paper } from '@mui/material';
import Register from '../components/Register';
import Login from '../components/Login';
import UserDetails from '../components/UserDetails';

const AuthPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4" gutterBottom>
        Страница аунтентификации
      </Typography>
      <Grid container spacing={3}>
        <Grid item xs={12} md={4}>
          <Paper elevation={3} style={{ padding: '16px' }}>
            <Typography variant="h5">Register</Typography>
            <Register />
          </Paper>
        </Grid>
        <Grid item xs={12} md={4}>
          <Paper elevation={3} style={{ padding: '16px' }}>
            <Typography variant="h5">Login</Typography>
            <Login />
          </Paper>
        </Grid>
        <Grid item xs={12} md={4}>
          <Paper elevation={3} style={{ padding: '16px' }}>
            <Typography variant="h5">User Details</Typography>
            <UserDetails />
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default AuthPage;

