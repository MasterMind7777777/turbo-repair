import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { loginUser } from '../services/auth';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const Login: React.FC = () => {
  const [email, setEmail] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { setToken } = useAuth();

  const handleLogin = async () => {
    try {
      const { token, user_id } = await loginUser(email, password);
      setToken(token, user_id);
      setResponse('Login successful');
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to login');
      }
    }
  };

  return (
    <Container>
      <TextField
        label="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Password"
        type="password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleLogin}>
        Login
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default Login;

