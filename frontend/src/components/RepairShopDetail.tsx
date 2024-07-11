import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import {
  Container,
  Typography,
  List,
  ListItem,
  ListItemText,
  Button,
  TextField,
  Paper,
  Modal,
  Box,
  IconButton,
} from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { getRepairShopById } from '../services/repairShop';
import { createUserRequest, getUserRequests } from '../services/userRequestService';
import { createStaffTask, getStaffTasks, linkStaffToTask } from '../services/staffTaskService';
import { RepairShopResponse, UserRequestListResponse, StaffTaskListResponse } from '../services/types';

const RepairShopDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [repairShop, setRepairShop] = useState<RepairShopResponse | null>(null);
  const [userRequests, setUserRequests] = useState<UserRequestListResponse[]>([]);
  const [staffTasks, setStaffTasks] = useState<StaffTaskListResponse[]>([]);
  const [userRequestContent, setUserRequestContent] = useState<string>('');
  const [staffTaskContent, setStaffTaskContent] = useState<string>('');
  const [selectedStaffIds, setSelectedStaffIds] = useState<string[]>(['']);
  const [staffTaskIdToLink, setStaffTaskIdToLink] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  useEffect(() => {
    const fetchRepairShop = async () => {
      if (id) {
        const shop = await getRepairShopById(id);
        setRepairShop(shop);
      }
    };

    const fetchUserRequests = async () => {
      const requests = await getUserRequests();
      setUserRequests(requests);
    };

    const fetchStaffTasks = async () => {
      const tasks = await getStaffTasks();
      setStaffTasks(tasks);
    };

    fetchRepairShop();
    fetchUserRequests();
    fetchStaffTasks();
  }, [id]);

  const handleCreateUserRequest = async () => {
    if (id && userRequestContent) {
      await createUserRequest(id, userRequestContent);
      setUserRequestContent('');
      const requests = await getUserRequests();
      setUserRequests(requests);
    }
  };

  const handleCreateStaffTask = async () => {
    if (id && staffTaskContent) {
      await createStaffTask(id, staffTaskContent, []);
      setStaffTaskContent('');
      const tasks = await getStaffTasks();
      setStaffTasks(tasks);
    }
  };

  const handleLinkStaffToTask = async () => {
    if (staffTaskIdToLink && selectedStaffIds.some(id => id.trim() !== '')) {
      await linkStaffToTask(staffTaskIdToLink, selectedStaffIds.filter(id => id.trim() !== ''));
      setSelectedStaffIds(['']);
      setStaffTaskIdToLink(null);
      const tasks = await getStaffTasks();
      setStaffTasks(tasks);
      setIsModalOpen(false);
    }
  };

  const handleAddStaffField = () => {
    setSelectedStaffIds([...selectedStaffIds, '']);
  };

  const handleStaffIdChange = (index: number, value: string) => {
    const newStaffIds = [...selectedStaffIds];
    newStaffIds[index] = value;
    setSelectedStaffIds(newStaffIds);
  };

  return (
    <Container>
      {repairShop && (
        <>
          <Typography variant="h4">{repairShop.name}</Typography>
          <Typography variant="h6">Запросы пользователей</Typography>
          <List>
            {userRequests.map((request) => (
              <ListItem key={request.id} component={Paper} elevation={1}>
                <ListItemText primary={request.content} />
              </ListItem>
            ))}
          </List>
          <TextField
            label="Новый запрос пользователя"
            value={userRequestContent}
            onChange={(e) => setUserRequestContent(e.target.value)}
            fullWidth
            margin="normal"
          />
          <Button variant="contained" color="primary" onClick={handleCreateUserRequest}>
            Отправить запрос пользователя
          </Button>

          <Typography variant="h6">Задачи для сотрудников</Typography>
          <List>
            {staffTasks.map((task) => (
              <ListItem key={task.id} component={Paper} elevation={1}>
                <ListItemText primary={task.content} />
                <Button
                  variant="outlined"
                  onClick={() => {
                    setStaffTaskIdToLink(task.id);
                    setIsModalOpen(true);
                  }}
                >
                  Назначить сотрудника
                </Button>
              </ListItem>
            ))}
          </List>
          <TextField
            label="Новая задача для сотрудника"
            value={staffTaskContent}
            onChange={(e) => setStaffTaskContent(e.target.value)}
            fullWidth
            margin="normal"
          />
          <Button variant="contained" color="primary" onClick={handleCreateStaffTask}>
            Отправить задачу сотруднику
          </Button>

          <Modal open={isModalOpen} onClose={() => setIsModalOpen(false)}>
            <Box
              sx={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                transform: 'translate(-50%, -50%)',
                width: 400,
                bgcolor: 'background.paper',
                boxShadow: 24,
                p: 4,
              }}
            >
              <Typography variant="h6">Назначить сотрудника на задачу</Typography>
              {selectedStaffIds.map((staffId, index) => (
                <TextField
                  key={index}
                  label={`ID сотрудника ${index + 1}`}
                  value={staffId}
                  onChange={(e) => handleStaffIdChange(index, e.target.value)}
                  fullWidth
                  margin="normal"
                />
              ))}
              <IconButton onClick={handleAddStaffField} color="primary">
                <AddIcon />
              </IconButton>
              <Button
                variant="contained"
                color="primary"
                onClick={handleLinkStaffToTask}
                fullWidth
              >
                Назначить сотрудника
              </Button>
            </Box>
          </Modal>
        </>
      )}
    </Container>
  );
};

export default RepairShopDetail;
