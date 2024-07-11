import React, { useState, useEffect } from 'react';
import {
  TextField,
  Button,
  Container,
  Typography,
  List,
  ListItem,
  ListItemText,
  Paper,
} from '@mui/material';
import { getOrders, updateOrderStatus } from '../services/order';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';
import { OrderResponse } from '../services/types';

const Order: React.FC = () => {
  const [orders, setOrders] = useState<OrderResponse[]>([]);
  const [status, setStatus] = useState<string>('in_progress');
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const fetchOrders = async () => {
    try {
      const orderList = await getOrders();
      setOrders(orderList);
    } catch (error) {
      console.error('Ошибка при получении заказов:', error);
    }
  };

  useEffect(() => {
    if (token) {
      fetchOrders();
    }
  }, [token]);

  const handleUpdateOrderStatus = async (id: string, newStatus: string) => {
    if (!token) {
      setResponse('Ошибка: не авторизован');
      return;
    }
    try {
      await updateOrderStatus(id, newStatus);
      setResponse(`Статус заказа обновлен: ${id}`);
      fetchOrders(); // Refresh the list after status update
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось обновить статус заказа');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Заказы</Typography>
      <List>
        {orders.map(order => (
          <ListItem key={order.id} component={Paper} elevation={1}>
            <ListItemText
              primary={`ID заказа: ${order.id}`}
              secondary={`Статус: ${order.status} | Создан: ${new Date(order.created_at).toLocaleString()}`}
            />
            <TextField
              label="Новый статус"
              value={status}
              onChange={(e) => setStatus(e.target.value)}
              fullWidth
              margin="normal"
            />
            <Button variant="contained" color="primary" onClick={() => handleUpdateOrderStatus(order.id, status)}>
              Обновить статус
            </Button>
          </ListItem>
        ))}
      </List>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default Order;
