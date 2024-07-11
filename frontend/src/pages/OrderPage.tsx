import React from 'react';
import { Container, Typography } from '@mui/material';
import Order from '../components/Order';

const OrderPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Страница заказа</Typography>
      <Order />
    </Container>
  );
};

export default OrderPage;

