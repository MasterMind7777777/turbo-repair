import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { registerUser } from '../services/auth';
import axios from 'axios';
import { useAuth } from '../context/AuthContext';

const Register: React.FC = () => {
  const [email, setEmail] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { setToken } = useAuth();

  const handleRegister = async () => {
    try {
      const { user_id, token } = await registerUser(email, password);
      setToken(token, user_id);
      setResponse(`Success: ${user_id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to register');
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
      <Button variant="contained" color="primary" onClick={handleRegister}>
        Register
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default Register;

