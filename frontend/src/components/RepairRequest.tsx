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
      console.error('Ошибка при получении запросов на ремонт:', error);
    }
  };

  const fetchBidsForRequest = async (requestId: string) => {
    try {
      const bids = await getBidsForRequest(requestId);
      setSelectedRequestBids(bids);
      setBidsDialogOpen(true);
    } catch (error) {
      console.error('Ошибка при получении заявок:', error);
    }
  };

  const handleAcceptBid = async (bidId: string) => {
    try {
      await acceptBid(bidId);
      setResponse('Заявка принята и заказ создан.');
      setBidsDialogOpen(false);
      fetchRepairRequests(); // Refresh the list to show updated state
    } catch (error) {
      console.error('Ошибка при принятии заявки:', error);
      setResponse('Ошибка при принятии заявки.');
    }
  };

  useEffect(() => {
    if (token) {
      fetchRepairRequests();
    }
  }, [token]);

  const handleRepairRequest = async () => {
    if (!token || !userId) {
      setResponse('Ошибка: не авторизован');
      return;
    }
    try {
      const { id } = await submitRepairRequest(userId, description);
      setResponse(`Запрос на ремонт создан: ${id}`);
      fetchRepairRequests(); // Refresh the list after submission
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось отправить запрос на ремонт');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Отправить запрос на ремонт</Typography>
      <TextField
        label="Описание"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleRepairRequest}>
        Отправить запрос
      </Button>
      {response && <Typography>{response}</Typography>}

      <Typography variant="h5" style={{ marginTop: '2rem' }}>Ваши запросы на ремонт</Typography>
      <List>
        {repairRequests.map(request => (
          <ListItem key={request.id} component={Paper} elevation={1}>
            <ListItemText
              primary={request.description}
              secondary={`Отправлено: ${new Date(request.created_at).toLocaleString()} | Заявки: ${request.bidCount}`}
            />
            <Button variant="contained" color="primary" onClick={() => fetchBidsForRequest(request.id)}>
              Просмотреть заявки
            </Button>
          </ListItem>
        ))}
      </List>

      <Dialog open={bidsDialogOpen} onClose={() => setBidsDialogOpen(false)} fullWidth maxWidth="md">
        <DialogTitle>Заявки на выбранный запрос на ремонт</DialogTitle>
        <DialogContent>
          {selectedRequestBids.length > 0 ? (
            <List>
              {selectedRequestBids.map(bid => (
                <ListItem key={bid.id}>
                  <ListItemText
                    primary={`Сумма заявки: ${bid.bid_amount}`}
                    secondary={`Статус: ${bid.status} | Отправлено: ${new Date(bid.created_at).toLocaleString()}`}
                  />
                  <Button variant="contained" color="primary" onClick={() => handleAcceptBid(bid.id)}>
                    Принять заявку
                  </Button>
                </ListItem>
              ))}
            </List>
          ) : (
            <Typography>Заявки для этого запроса не найдены.</Typography>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setBidsDialogOpen(false)} color="primary">
            Закрыть
          </Button>
        </DialogActions>
      </Dialog>
    </Container>
  );
};

export default RepairRequest;
