import React, { useState, useEffect } from 'react';
import { TextField, Button, Container, Typography, Grid, Paper, Box, MenuItem, Select, FormControl, InputLabel } from '@mui/material';
import { submitBid } from '../services/bid';
import { getAvailableRequests } from '../services/repairRequest';
import { getRepairShops } from '../services/repairShop';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';
import { RepairRequestListResponse, RepairShopResponse } from '../services/types';

const Bid: React.FC = () => {
  const [availableRequests, setAvailableRequests] = useState<RepairRequestListResponse[]>([]);
  const [selectedRequest, setSelectedRequest] = useState<RepairRequestListResponse | null>(null);
  const [repairShops, setRepairShops] = useState<RepairShopResponse[]>([]);
  const [selectedRepairShopId, setSelectedRepairShopId] = useState<string>('');
  const [status, setStatus] = useState<string>('pending');
  const [bidAmount, setBidAmount] = useState<number>(0);
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  useEffect(() => {
    if (token) {
      fetchAvailableRequests();
      fetchRepairShops();
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

  const fetchRepairShops = async () => {
    try {
      const shops = await getRepairShops();
      setRepairShops(shops);
    } catch (error) {
      console.error('Error fetching repair shops:', error);
    }
  };

  const handleBid = async () => {
    if (!token || !selectedRequest || !selectedRepairShopId) {
      setResponse('Error: Not authenticated or no request/repair shop selected');
      return;
    }
    try {
      const { id } = await submitBid(selectedRequest.id, selectedRepairShopId, status, bidAmount);
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
          <FormControl fullWidth margin="normal">
            <InputLabel id="repair-shop-label">Repair Shop</InputLabel>
            <Select
              labelId="repair-shop-label"
              value={selectedRepairShopId}
              onChange={(e) => setSelectedRepairShopId(e.target.value as string)}
            >
              {repairShops.map(shop => (
                <MenuItem key={shop.id} value={shop.id}>
                  {shop.name}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
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

