import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { addStatusPipelineEntry } from '../services/statusPipeline';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const StatusPipeline: React.FC = () => {
  const [orderId, setOrderId] = useState<string>('');
  const [status, setStatus] = useState<string>('in_progress');
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const handleAddStatusPipelineEntry = async () => {
    if (!token) {
      setResponse('Ошибка: не авторизован');
      return;
    }
    try {
      const { id } = await addStatusPipelineEntry(orderId, status);
      setResponse(`Запись в статус пайплайн добавлена: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось добавить запись в статус пайплайн');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Добавить запись в статус пайплайн</Typography>
      <TextField
        label="ID заказа"
        value={orderId}
        onChange={(e) => setOrderId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Статус"
        value={status}
        onChange={(e) => setStatus(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleAddStatusPipelineEntry}>
        Добавить запись
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default StatusPipeline;
