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
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await addStatusPipelineEntry(orderId, status);
      setResponse(`Status pipeline entry added: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to add status pipeline entry');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Add Status Pipeline Entry</Typography>
      <TextField
        label="Order ID"
        value={orderId}
        onChange={(e) => setOrderId(e.target.value)}
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
      <Button variant="contained" color="primary" onClick={handleAddStatusPipelineEntry}>
        Add Entry
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default StatusPipeline;

