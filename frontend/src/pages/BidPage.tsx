import React from 'react';
import { Container, Typography } from '@mui/material';
import Bid from '../components/Bid';

const BidPage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Submit Bid</Typography>
      <Bid />
    </Container>
  );
};

export default BidPage;

