import React from 'react';
import { Container, Typography } from '@mui/material';
import RepairShopManager from '../components/RepairShop';

const RepairShopPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Страница упарвления мастерскими</Typography>
      <RepairShopManager />
    </Container>
  );
};

export default RepairShopPage;

