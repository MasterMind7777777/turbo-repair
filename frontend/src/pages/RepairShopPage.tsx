import React from 'react';
import { Container, Typography } from '@mui/material';
import RepairShopManager from '../components/RepairShop';

const RepairShopPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Create Repair Shop</Typography>
      <RepairShopManager />
    </Container>
  );
};

export default RepairShopPage;

