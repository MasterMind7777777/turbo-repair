import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { addStaff } from '../services/staff';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const AddStaffForm: React.FC = () => {
  const [userId, setUserId] = useState<string>('');
  const [repairShopId, setRepairShopId] = useState<string>('');
  const [role, setRole] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const handleAddStaff = async () => {
    if (!token) {
      setResponse('Ошибка: не авторизован');
      return;
    }
    try {
      const { id } = await addStaff(userId, repairShopId, role);
      setResponse(`Персонал успешно добавлен: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось добавить персонал');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Добавить Персонал</Typography>
      <TextField
        label="ID Пользователя"
        value={userId}
        onChange={(e) => setUserId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="ID Ремонтной Мастерской"
        value={repairShopId}
        onChange={(e) => setRepairShopId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Роль"
        value={role}
        onChange={(e) => setRole(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleAddStaff}>
        Добавить Персонал
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default AddStaffForm;
