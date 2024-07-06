import React, { useState } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { addStaff } from '../services/staff';
import { useAuth } from '../context/AuthContext';
import axios from 'axios';

const AddStaffForm: React.FC = () => {
  const [userId, setUserId] = useState<string>('');
  const [repairShopId, setRepairShopId] = useState<string>('');
  const [role, setRole] = useState<string>('');
  const [response, setResponse] = useState<string>('');
  const { token } = useAuth();

  const handleAddStaff = async () => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }
    try {
      const { id } = await addStaff(userId, repairShopId, role);
      setResponse(`Staff added successfully: ${id}`);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to add staff');
      }
    }
  };

  return (
    <Container>
      <Typography variant="h4">Add Staff</Typography>
      <TextField
        label="User ID"
        value={userId}
        onChange={(e) => setUserId(e.target.value)}
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
        label="Role"
        value={role}
        onChange={(e) => setRole(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleAddStaff}>
        Add Staff
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default AddStaffForm;

