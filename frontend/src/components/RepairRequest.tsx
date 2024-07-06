import React, { useState, useEffect } from 'react';
import { TextField, Button, Container, Typography, List, ListItem, ListItemText } from '@mui/material';
import { submitRepairRequest, getRepairRequests } from '../services/repairRequest';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';
import { RepairRequestListResponse } from '../services/types';

const RepairRequest: React.FC = () => {
  const [description, setDescription] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const [repairRequests, setRepairRequests] = useState<RepairRequestListResponse[]>([]);
  const { token, userId } = useAuth();

  const fetchRepairRequests = async () => {
    try {
      const requests = await getRepairRequests();
      setRepairRequests(requests);
    } catch (error) {
      console.error('Error fetching repair requests:', error);
    }
  };

  useEffect(() => {
    if (token) {
      fetchRepairRequests();
    }
  }, [token]);

  const handleRepairRequest = async () => {
    if (!token || !userId) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await submitRepairRequest(userId, description);
      setResponse(`Repair request created: ${id}`);
      fetchRepairRequests(); // Refresh the list after submission
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

      <Typography variant="h5" style={{ marginTop: '2rem' }}>Your Repair Requests</Typography>
      <List>
        {repairRequests.map(request => (
          <ListItem key={request.id}>
            <ListItemText
              primary={request.description}
              secondary={`Submitted on: ${new Date(request.created_at).toLocaleString()}`}
            />
          </ListItem>
        ))}
      </List>
    </Container>
  );
};

export default RepairRequest;

