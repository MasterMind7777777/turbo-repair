import React, { useState, useEffect } from 'react';
import { TextField, Button, Container, Typography } from '@mui/material';
import { addAddress, updateAddress } from '../services/address';
import { AddressResponse } from '../services/types';

interface AddressFormProps {
  repairShopId: string;
  addressId?: string | null;
  existingAddress?: AddressResponse | null;
  onAddressAddedOrUpdated: (address: AddressResponse) => void;
}

const AddressForm: React.FC<AddressFormProps> = ({ repairShopId, addressId, existingAddress, onAddressAddedOrUpdated }) => {
  const [street, setStreet] = useState<string>('');
  const [city, setCity] = useState<string>('');
  const [state, setState] = useState<string>('');
  const [zip, setZip] = useState<string>('');
  const [country, setCountry] = useState<string>('');
  const [response, setResponse] = useState<string>('');

  useEffect(() => {
    if (existingAddress) {
      setStreet(existingAddress.street);
      setCity(existingAddress.city);
      setState(existingAddress.state);
      setZip(existingAddress.zip);
      setCountry(existingAddress.country);
    }
  }, [existingAddress]);

  const handleSaveAddress = async () => {
    try {
      let address: AddressResponse;
      if (addressId) {
        address = await updateAddress(addressId, street, city, state, zip, country);
        setResponse('Адрес успешно обновлён');
      } else {
        address = await addAddress(repairShopId, street, city, state, zip, country);
        setResponse('Адрес успешно добавлен');
      }
      onAddressAddedOrUpdated(address);
      setStreet('');
      setCity('');
      setState('');
      setZip('');
      setCountry('');
    } catch (error) {
      setResponse('Ошибка: не удалось сохранить адрес');
    }
  };

  return (
    <Container>
      <Typography variant="h6">{addressId ? 'Обновить Адрес' : 'Добавить Адрес'}</Typography>
      <TextField
        label="Улица"
        value={street}
        onChange={(e) => setStreet(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Город"
        value={city}
        onChange={(e) => setCity(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Штат/Область"
        value={state}
        onChange={(e) => setState(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Индекс"
        value={zip}
        onChange={(e) => setZip(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Страна"
        value={country}
        onChange={(e) => setCountry(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleSaveAddress}>
        {addressId ? 'Обновить Адрес' : 'Добавить Адрес'}
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default AddressForm;
