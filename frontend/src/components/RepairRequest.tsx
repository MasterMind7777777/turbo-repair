import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { submitRepairRequest } from '../services/repairRequest';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const RepairRequest: React.FC = () => {
  const [customerId, setCustomerId] = useState<string>('');
  const [description, setDescription] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const handleRepairRequest = async () => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await submitRepairRequest(customerId, description);
      setResponse(`Repair request created: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to submit repair request');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Submit Repair Request</Typography>
      <TextField
        label="Customer ID"
        value={customerId}
        onChange={(e) => setCustomerId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Description"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleRepairRequest}>
        Submit Request
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default RepairRequest;

