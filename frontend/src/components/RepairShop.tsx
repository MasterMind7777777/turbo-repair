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

const RepairShopManager: React.FC = () => {
  const [name, setName] = useState<string>('');
  const [selectedShopId, setSelectedShopId] = useState<string | null>(null);
  const [repairShops, setRepairShops] = useState<RepairShopResponse[]>([]);
  const [selectedAddress, setSelectedAddress] = useState<AddressResponse | null>(null);
  const [response, setResponse] = useState<string>('');
  const [showAddressForm, setShowAddressForm] = useState<boolean>(false);
  const { token } = useAuth();

  useEffect(() => {
    const fetchRepairShops = async () => {
      try {
        const shops = await getRepairShops();
        setRepairShops(shops);
      } catch (error) {
        console.error('Error fetching repair shops:', error);
      }
    };
    fetchRepairShops();
  }, []);

  const handleCreateOrUpdateShop = async () => {
    if (!token) {
      setResponse('Error: Not authenticated');
      return;
    }

    try {
      let shopId: string;
      if (selectedShopId) {
        await updateRepairShop(selectedShopId, name);
        setResponse(`Shop updated: ${selectedShopId}`);
        shopId = selectedShopId;
      } else {
        const { id } = await createRepairShop(name);
        setResponse(`Shop created: ${id}`);
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
        setResponse(`Error: ${error.response.data}`);
      } else {
        setResponse('Error: Unable to create/update shop');
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
      console.error('Error fetching address:', error);
    }
    setShowAddressForm(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await deleteRepairShop(id);
      setRepairShops(repairShops.filter(shop => shop.id !== id));
      setResponse(`Shop deleted: ${id}`);
    } catch (error) {
      setResponse('Error: Unable to delete shop');
    }
  };

  const handleAddressAddedOrUpdated = (address: AddressResponse) => {
    setSelectedAddress(address);
  };

  return (
    <Container>
      <Typography variant="h4">{selectedShopId ? 'Edit' : 'Create'} Repair Shop</Typography>
      <TextField
        label="Shop Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleCreateOrUpdateShop}>
        {selectedShopId ? 'Update Shop' : 'Create Shop'}
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
        {showAddressForm ? 'Hide Address Form' : 'Add Address'}
      </Button>

      <Collapse in={showAddressForm && !selectedShopId}>
        <AddressForm
          repairShopId={selectedShopId ?? ''}
          addressId={null}
          existingAddress={null}
          onAddressAddedOrUpdated={handleAddressAddedOrUpdated}
        />
      </Collapse>

      <Typography variant="h4">Repair Shops</Typography>
      <List>
        {repairShops.map((shop) => (
          <ListItem key={shop.id} component={Paper} elevation={selectedShopId === shop.id ? 4 : 1}>
            <ListItemText primary={shop.name} />
            <Button variant="contained" color="primary" onClick={() => handleEdit(shop)}>
              Edit
            </Button>
            <Button variant="contained" color="secondary" onClick={() => handleDelete(shop.id)}>
              Delete
            </Button>
          </ListItem>
        ))}
      </List>
    </Container>
  );
};

export default RepairShopManager;
