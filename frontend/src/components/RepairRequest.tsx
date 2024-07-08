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
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from '@mui/material';
import { submitRepairRequest, getAvailableRequests } from '../services/repairRequest';
import { getBidsForRequest, acceptBid } from '../services/bid';  // Import acceptBid function
import { useAuth } from '../context/AuthContext';
import axios from 'axios';
import { RepairRequestListResponse, BidResponse } from '../services/types';

const RepairRequest: React.FC = () => {
  const [description, setDescription] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const [repairRequests, setRepairRequests] = useState<(RepairRequestListResponse & { bidCount: number })[]>([]);
  const [selectedRequestBids, setSelectedRequestBids] = useState<BidResponse[]>([]);
  const [bidsDialogOpen, setBidsDialogOpen] = useState<boolean>(false);
  const { token, userId } = useAuth();

  const fetchRepairRequests = async () => {
    try {
      const requests = await getAvailableRequests ();
      const requestsWithBidCounts = await Promise.all(
        requests.map(async (request) => {
          const bids = await getBidsForRequest(request.id);
          return { ...request, bidCount: bids.length };
        })
      );
      setRepairRequests(requestsWithBidCounts);
    } catch (error) {
      console.error('Error fetching repair requests:', error);
    }
  };

  const fetchBidsForRequest = async (requestId: string) => {
    try {
      const bids = await getBidsForRequest(requestId);
      setSelectedRequestBids(bids);
      setBidsDialogOpen(true);
    } catch (error) {
      console.error('Error fetching bids:', error);
    }
  };

  const handleAcceptBid = async (bidId: string) => {
    try {
      await acceptBid(bidId);
      setResponse('Bid accepted and order created.');
      setBidsDialogOpen(false);
      fetchRepairRequests(); // Refresh the list to show updated state
    } catch (error) {
      console.error('Error accepting bid:', error);
      setResponse('Error accepting bid.');
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
          <ListItem key={request.id} component={Paper} elevation={1}>
            <ListItemText
              primary={request.description}
              secondary={`Submitted on: ${new Date(request.created_at).toLocaleString()} | Bids: ${request.bidCount}`}
            />
            <Button variant="contained" color="primary" onClick={() => fetchBidsForRequest(request.id)}>
              View Bids
            </Button>
          </ListItem>
        ))}
      </List>

      <Dialog open={bidsDialogOpen} onClose={() => setBidsDialogOpen(false)} fullWidth maxWidth="md">
        <DialogTitle>Bids for Selected Repair Request</DialogTitle>
        <DialogContent>
          {selectedRequestBids.length > 0 ? (
            <List>
              {selectedRequestBids.map(bid => (
                <ListItem key={bid.id}>
                  <ListItemText
                    primary={`Bid Amount: ${bid.bid_amount}`}
                    secondary={`Status: ${bid.status} | Submitted on: ${new Date(bid.created_at).toLocaleString()}`}
                  />
                  <Button variant="contained" color="primary" onClick={() => handleAcceptBid(bid.id)}>
                    Accept Bid
                  </Button>
                </ListItem>
              ))}
            </List>
          ) : (
            <Typography>No bids found for this request.</Typography>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setBidsDialogOpen(false)} color="primary">
            Close
          </Button>
        </DialogActions>
      </Dialog>
    </Container>
  );
};

export default RepairRequest;
