import React from 'react';
import { Container, Typography } from '@mui/material';
import AddStaffForm from '../components/AddStaffForm';

const StaffPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Manage staff</Typography>
      <AddStaffForm />
    </Container>
  );
};

export default StaffPage;


