import React from 'react';
import { AppBar, Toolbar, Typography, Button } from '@mui/material';
import { Link } from 'react-router-dom';

const Navbar: React.FC = () => {
  return (
    <AppBar position="static">
      <Toolbar>
        <Typography variant="h6" style={{ flexGrow: 1 }}>
          Управление Ремонтной Мастерской
        </Typography>
        <Button color="inherit" component={Link} to="/">
          Авторизация
        </Button>
        <Button color="inherit" component={Link} to="/repair-shop">
          Ремонтная Мастерская
        </Button>
        <Button color="inherit" component={Link} to="/repair-request">
          Запрос на Ремонт
        </Button>
        <Button color="inherit" component={Link} to="/bid">
          Приём заявок
        </Button>
        <Button color="inherit" component={Link} to="/order">
          Заказ
        </Button>
        <Button color="inherit" component={Link} to="/status-pipeline">
          Статус Пайплайн
        </Button>
        <Button color="inherit" component={Link} to="/staff">
          Персонал
        </Button>
      </Toolbar>
    </AppBar>
  );
};

export default Navbar;
