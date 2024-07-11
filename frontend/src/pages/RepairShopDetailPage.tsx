import React from 'react';
import { Container, Typography } from '@mui/material';
import RepairShopDetail from '../components/RepairShopDetail';

const RepairShopDetailPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Страница конкретной мастерской</Typography>
      <RepairShopDetail  />
    </Container>
  );
};

export default RepairShopDetailPage;

