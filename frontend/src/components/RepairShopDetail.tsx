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
} from '@mui/material';
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
  const [selectedStaffIds, setSelectedStaffIds] = useState<string[]>([]);
  const [staffTaskIdToLink, setStaffTaskIdToLink] = useState<string | null>(null);

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
    if (staffTaskIdToLink && selectedStaffIds.length > 0) {
      await linkStaffToTask(staffTaskIdToLink, selectedStaffIds);
      setSelectedStaffIds([]);
      setStaffTaskIdToLink(null);
      const tasks = await getStaffTasks();
      setStaffTasks(tasks);
    }
  };

  return (
    <Container>
      {repairShop && (
        <>
          <Typography variant="h4">{repairShop.name}</Typography>
          <Typography variant="h6">User Requests</Typography>
          <List>
            {userRequests.map((request) => (
              <ListItem key={request.id} component={Paper} elevation={1}>
                <ListItemText primary={request.content} />
              </ListItem>
            ))}
          </List>
          <TextField
            label="New User Request"
            value={userRequestContent}
            onChange={(e) => setUserRequestContent(e.target.value)}
            fullWidth
            margin="normal"
          />
          <Button variant="contained" color="primary" onClick={handleCreateUserRequest}>
            Submit User Request
          </Button>

          <Typography variant="h6">Staff Tasks</Typography>
          <List>
            {staffTasks.map((task) => (
              <ListItem key={task.id} component={Paper} elevation={1}>
                <ListItemText primary={task.content} />
                <Button
                  variant="outlined"
                  onClick={() => setStaffTaskIdToLink(task.id)}
                >
                  Link Staff
                </Button>
              </ListItem>
            ))}
          </List>
          <TextField
            label="New Staff Task"
            value={staffTaskContent}
            onChange={(e) => setStaffTaskContent(e.target.value)}
            fullWidth
            margin="normal"
          />
          <Button variant="contained" color="primary" onClick={handleCreateStaffTask}>
            Submit Staff Task
          </Button>

          {staffTaskIdToLink && (
            <>
              <Typography variant="h6">Link Staff to Task</Typography>
              <TextField
                label="Staff ID"
                value={selectedStaffIds.join(', ')}
                onChange={(e) => setSelectedStaffIds(e.target.value.split(',').map(id => id.trim()))}
                fullWidth
                margin="normal"
              />
              <Button variant="contained" color="primary" onClick={handleLinkStaffToTask}>
                Link Staff
              </Button>
            </>
          )}
        </>
      )}
    </Container>
  );
};

export default RepairShopDetail;

