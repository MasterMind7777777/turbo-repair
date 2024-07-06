import React, { useState, useEffect } from 'react';
import { TextField, Button, Container, Typography, Grid, Paper, Box } from '@mui/material';
import { submitBid } from '../services/bid';
import { getAvailableRequests } from '../services/repairRequest';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';
import { RepairRequestListResponse } from '../services/types';

const Bid: React.FC = () => {
  const [availableRequests, setAvailableRequests] = useState<RepairRequestListResponse[]>([]);
  const [selectedRequest, setSelectedRequest] = useState<RepairRequestListResponse | null>(null);
  const [repairShopId, setRepairShopId] = useState<string>('');
  const [status, setStatus] = useState<string>('pending');
  const [bidAmount, setBidAmount] = useState<number>(0);
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  useEffect(() => {
    if (token) {
      fetchAvailableRequests();
    }
  }, [token]);

  const fetchAvailableRequests = async () => {
    try {
      const requests = await getAvailableRequests();
      setAvailableRequests(requests);
    } catch (error) {
      console.error('Error fetching available requests:', error);
    }
  };

  const handleBid = async () => {
    if (!token || !selectedRequest) {
      setResponse('Error: Not authenticated or no request selected');
      return;
    }
    try {
      const { id } = await submitBid(selectedRequest.id, repairShopId, status, bidAmount);
      setResponse(`Bid submitted successfully: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to submit bid');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Available Repair Requests</Typography>
      <Grid container spacing={2}>
        {availableRequests.map(request => (
          <Grid item xs={12} sm={6} md={4} key={request.id}>
            <Paper
              elevation={3}
              style={{ padding: '16px', cursor: 'pointer' }}
              onClick={() => setSelectedRequest(request)}
            >
              <Typography variant="h6">{request.description}</Typography>
              <Typography variant="body2">
                Submitted on: {new Date(request.created_at).toLocaleString()}
              </Typography>
              <Typography variant="body2">Request ID: {request.id}</Typography>
            </Paper>
          </Grid>
        ))}
      </Grid>
      {selectedRequest && (
        <Box mt={4}>
          <Typography variant="h4">Submit Bid</Typography>
          <Typography variant="h6">
            Repair Request: {selectedRequest.description} (ID: {selectedRequest.id})
          </Typography>
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
          <TextField
            label="Bid Amount"
            type="number"
            value={bidAmount}
            onChange={(e) => setBidAmount(Number(e.target.value))}
            fullWidth
            margin="normal"
          />
          <Button variant="contained" color="primary" onClick={handleBid}>
            Submit Bid
          </Button>
          {response && <Typography>{response}</Typography>}
        </Box>
      )}
    </Container>
  );
};

export default Bid;

