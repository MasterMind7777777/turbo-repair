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
      console.error('Error fetching orders:', error);
    }
  };

  useEffect(() => {
    if (token) {
      fetchOrders();
    }
  }, [token]);

  const handleUpdateOrderStatus = async (id: string, newStatus: string) => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      await updateOrderStatus(id, newStatus);
      setResponse(`Order status updated: ${id}`);
      fetchOrders(); // Refresh the list after status update
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to update order status');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Orders</Typography>
      <List>
        {orders.map(order => (
          <ListItem key={order.id} component={Paper} elevation={1}>
            <ListItemText
              primary={`Order ID: ${order.id}`}
              secondary={`Status: ${order.status} | Created at: ${new Date(order.created_at).toLocaleString()}`}
            />
            <TextField
              label="New Status"
              value={status}
              onChange={(e) => setStatus(e.target.value)}
              fullWidth
              margin="normal"
            />
            <Button variant="contained" color="primary" onClick={() => handleUpdateOrderStatus(order.id, status)}>
              Update Status
            </Button>
          </ListItem>
        ))}
      </List>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default Order;

