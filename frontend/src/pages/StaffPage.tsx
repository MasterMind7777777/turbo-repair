import React from 'react';
import { Container, Typography } from '@mui/material';
import AddStaffForm from '../components/AddStaffForm';

const StaffPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Страница управления персоналом</Typography>
      <AddStaffForm />
    </Container>
  );
};

export default StaffPage;


