import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import AuthPage from './pages/AuthPage';
import RepairShopPage from './pages/RepairShopPage';
import RepairRequestPage from './pages/RepairRequestPage';
import BidPage from './pages/BidPage';
import OrderPage from './pages/OrderPage';
import StatusPipelinePage from './pages/StatusPipelinePage';
import Navbar from './components/Navbar';

const AppRouter: React.FC = () => {
  return (
    <Router>
      <Navbar />
      <Routes>
        <Route path="/" element={<AuthPage />} />
        <Route path="/repair-shop" element={<RepairShopPage />} />
        <Route path="/repair-request" element={<RepairRequestPage />} />
        <Route path="/bid" element={<BidPage />} />
        <Route path="/order" element={<OrderPage />} />
        <Route path="/status-pipeline" element={<StatusPipelinePage />} />
      </Routes>
    </Router>
  );
};

export default AppRouter;

