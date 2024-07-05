import React from 'react';
import { Container, Typography, Grid, Paper } from '@mui/material';
import Register from '../components/Register';
import Login from '../components/Login';

const AuthPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4" gutterBottom>
        Authentication
      </Typography>
      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Paper elevation={3} style={{ padding: '16px' }}>
            <Typography variant="h5">Register</Typography>
            <Register />
          </Paper>
        </Grid>
        <Grid item xs={12} md={6}>
          <Paper elevation={3} style={{ padding: '16px' }}>
            <Typography variant="h5">Login</Typography>
            <Login />
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default AuthPage;

