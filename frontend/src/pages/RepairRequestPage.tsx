import React from 'react';
import { Container, Typography } from '@mui/material';
import RepairRequest from '../components/RepairRequest';

const RepairRequestPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Страница заявки</Typography>
      <RepairRequest />
    </Container>
  );
};

export default RepairRequestPage;

