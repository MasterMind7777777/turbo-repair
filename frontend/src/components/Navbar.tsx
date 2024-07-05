import React from 'react';
import { AppBar, Toolbar, Typography, Button } from '@mui/material';
import { Link } from 'react-router-dom';

const Navbar: React.FC = () => {
  return (
    <AppBar position="static">
      <Toolbar>
        <Typography variant="h6" style={{ flexGrow: 1 }}>
          Repair Shop Management
        </Typography>
        <Button color="inherit" component={Link} to="/">
          Auth
        </Button>
        <Button color="inherit" component={Link} to="/repair-shop">
          Repair Shop
        </Button>
        <Button color="inherit" component={Link} to="/repair-request">
          Repair Request
        </Button>
        <Button color="inherit" component={Link} to="/bid">
          Bid
        </Button>
        <Button color="inherit" component={Link} to="/order">
          Order
        </Button>
        <Button color="inherit" component={Link} to="/status-pipeline">
          Status Pipeline
        </Button>
      </Toolbar>
    </AppBar>
  );
};

export default Navbar;

