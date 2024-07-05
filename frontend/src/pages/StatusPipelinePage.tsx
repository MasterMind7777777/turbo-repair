import React from 'react';
import { Container, Typography } from '@mui/material';
import StatusPipeline from '../components/StatusPipeline';

const StatusPipelinePage: React.FC = () => {
  return (
    <Container>
      <Typography variant="h4">Status Pipeline</Typography>
      <StatusPipeline />
    </Container>
  );
};

export default StatusPipelinePage;

