import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { createOrder, updateOrderStatus } from '../services/order';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const Order: React.FC = () => {
  const [repairRequestId, setRepairRequestId] = useState<string>('');
  const [repairShopId, setRepairShopId] = useState<string>('');
  const [status, setStatus] = useState<string>('accepted');
  const [orderId, setOrderId] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const handleCreateOrder = async () => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await createOrder(repairRequestId, repairShopId, status);
      setOrderId(id);
      setResponse(`Order created successfully: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to create order');
      }
    }
  };

  const handleUpdateOrderStatus = async () => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await updateOrderStatus(orderId, 'in_progress');
      setResponse(`Order status updated: ${id}`);
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
      <Typography variant="h4">Create Order</Typography>
      <TextField
        label="Repair Request ID"
        value={repairRequestId}
        onChange={(e) => setRepairRequestId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Repair Shop ID"
        value={repairShopId}
        onChange={(e) => setRepairShopId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Status"
        value={status}
        onChange={(e) => setStatus(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleCreateOrder}>
        Create Order
      </Button>
      <Typography variant="h4">Update Order Status</Typography>
      <TextField
        label="Order ID"
        value={orderId}
        onChange={(e) => setOrderId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleUpdateOrderStatus}>
        Update Status
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default Order;

