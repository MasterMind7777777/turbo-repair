import React, { useState, useEffect } from 'react';
import {
  TextField,
  Button,
  Container,
  Typography,
  List,
  ListItem,
  ListItemText,
  Collapse,
  Paper,
} from '@mui/material';
import { createRepairShop, getRepairShops, updateRepairShop, deleteRepairShop } from '../services/repairShop';
import { useAuth } from '../context/AuthContext';
import { RepairShopResponse, AddressResponse } from '../services/types';
import AddressForm from './AddressForm';
import { getAddressByShopId } from '../services/address';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

const RepairShopManager: React.FC = () => {
  const [name, setName] = useState<string>('');
  const [selectedShopId, setSelectedShopId] = useState<string | null>(null);
  const [repairShops, setRepairShops] = useState<RepairShopResponse[]>([]);
  const [selectedAddress, setSelectedAddress] = useState<AddressResponse | null>(null);
  const [response, setResponse] = useState<string>('');
  const [showAddressForm, setShowAddressForm] = useState<boolean>(false);
  const { token } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    const fetchRepairShops = async () => {
      try {
        const shops = await getRepairShops();
        setRepairShops(shops);
      } catch (error) {
        console.error('Ошибка при получении ремонтных мастерских:', error);
      }
    };
    fetchRepairShops();
  }, []);

  const handleCreateOrUpdateShop = async () => {
    if (!token) {
      setResponse('Ошибка: не авторизован');
      return;
    }

    try {
      let shopId: string;
      if (selectedShopId) {
        await updateRepairShop(selectedShopId, name);
        setResponse(`Мастерская обновлена: ${selectedShopId}`);
        shopId = selectedShopId;
      } else {
        const { id } = await createRepairShop(name);
        setResponse(`Мастерская создана: ${id}`);
        shopId = id;
        handleEdit({ id, name, created_at: new Date().toISOString() } as RepairShopResponse);
      }

      setName('');
      setSelectedShopId(shopId);
      setSelectedAddress(null);
      setShowAddressForm(false);

      const shops = await getRepairShops();
      setRepairShops(shops);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        setResponse(`Ошибка: ${error.response.data}`);
      } else {
        setResponse('Ошибка: не удалось создать/обновить мастерскую');
      }
    }
  };

  const handleEdit = async (shop: RepairShopResponse) => {
    setName(shop.name);
    setSelectedShopId(shop.id);
    try {
      const address = await getAddressByShopId(shop.id);
      if (address) {
        setSelectedAddress(address);
      } else {
        setSelectedAddress(null);
      }
    } catch (error) {
      console.error('Ошибка при получении адреса:', error);
    }
    setShowAddressForm(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await deleteRepairShop(id);
      setRepairShops(repairShops.filter(shop => shop.id !== id));
      setResponse(`Мастерская удалена: ${id}`);
    } catch (error) {
      setResponse('Ошибка: не удалось удалить мастерскую');
    }
  };

  const handleAddressAddedOrUpdated = (address: AddressResponse) => {
    setSelectedAddress(address);
  };

  const handleGoToRepairShopPage = (id: string) => {
    navigate(`/repair-shop/${id}`);
  };

  return (
    <Container>
      <Typography variant="h4">{selectedShopId ? 'Редактировать' : 'Создать'} Ремонтную Мастерскую</Typography>
      <TextField
        label="Название мастерской"
        value={name}
        onChange={(e) => setName(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleCreateOrUpdateShop}>
        {selectedShopId ? 'Обновить мастерскую' : 'Создать мастерскую'}
      </Button>
      {response && <Typography>{response}</Typography>}

      {selectedShopId && (
        <AddressForm
          repairShopId={selectedShopId}
          addressId={selectedAddress?.id ?? null}
          existingAddress={selectedAddress}
          onAddressAddedOrUpdated={handleAddressAddedOrUpdated}
        />
      )}

      <Button variant="text" color="primary" onClick={() => setShowAddressForm(!showAddressForm)}>
        {showAddressForm ? 'Скрыть форму адреса' : 'Добавить адрес'}
      </Button>

      <Collapse in={showAddressForm && !selectedShopId}>
        <AddressForm
          repairShopId={selectedShopId ?? ''}
          addressId={null}
          existingAddress={null}
          onAddressAddedOrUpdated={handleAddressAddedOrUpdated}
        />
      </Collapse>

      <Typography variant="h4">Ремонтные Мастерские</Typography>
      <List>
        {repairShops.map((shop) => (
          <ListItem key={shop.id} component={Paper} elevation={selectedShopId === shop.id ? 4 : 1}>
            <ListItemText primary={shop.name} />
            <Button variant="contained" color="primary" onClick={() => handleEdit(shop)}>
              Редактировать
            </Button>
            <Button variant="contained" color="secondary" onClick={() => handleDelete(shop.id)}>
              Удалить
            </Button>
            <Button variant="contained" color="secondary" onClick={() => handleGoToRepairShopPage(shop.id)}>
              Перейти на страницу мастерской
            </Button>
          </ListItem>
        ))}
      </List>
    </Container>
  );
};

export default RepairShopManager;
