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
      setResponse('Вход выполнен успешно');
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось войти');
      }
    }
  };

  return (
    <Container>
      <TextField
        label="Электронная почта"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Пароль"
        type="password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleLogin}>
        Войти
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default Login;
