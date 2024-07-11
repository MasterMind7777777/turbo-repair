import React, { useState, useEffect } from 'react';
import { TextField, Button, Container, Typography, Paper } from '@mui/material';
import { getCurrentUser, updateUserProfile } from '../services/user';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const UserDetails: React.FC = () => {
  const [email, setEmail] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { token, userId } = useAuth();

  useEffect(() => {
    if (token) {
      getCurrentUser().then(user => {
        setEmail(user.email);
      }).catch(error => {
        console.error('Ошибка при получении данных пользователя:', error);
      });
    }
  }, [token]);

  const handleUpdateProfile = async () => {
    if (!userId) {
      setResponse('Ошибка: не авторизован');
      return;
    }
    try {
      await updateUserProfile(userId, { email });
      setResponse('Профиль успешно обновлен');
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось обновить профиль');
      }
    }
  };

  return (
    <Container>
      <Typography variant="body1">id: {userId}</Typography>
      <Paper elevation={3} style={{ padding: '16px', marginTop: '16px' }}>
        <TextField
          label="Электронная почта"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          fullWidth
          margin="normal"
        />
        <Button variant="contained" color="primary" onClick={handleUpdateProfile}>
          Обновить профиль
        </Button>
        {response && <Typography>{response}</Typography>}
      </Paper>
    </Container>
  );
};

export default UserDetails;
