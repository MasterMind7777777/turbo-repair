import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { submitBid } from '../services/bid';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const Bid: React.FC = () => {
  const [repairRequestId, setRepairRequestId] = useState<string>('');
  const [repairShopId, setRepairShopId] = useState<string>('');
  const [status, setStatus] = useState<string>('pending');
  const [bidAmount, setBidAmount] = useState<number>(0);
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const handleBid = async () => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await submitBid(repairRequestId, repairShopId, status, bidAmount);
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
      <Typography variant="h4">Submit Bid</Typography>
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
    </Container>
  );
};

export default Bid;

