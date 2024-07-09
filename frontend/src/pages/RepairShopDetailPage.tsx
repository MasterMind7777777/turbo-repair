import React from 'react';
import { Container, Typography } from '@mui/material';
import RepairShopDetail from '../components/RepairShopDetail';

const RepairShopDetailPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Repair Shop Detail</Typography>
      <RepairShopDetail  />
    </Container>
  );
};

export default RepairShopDetailPage;

