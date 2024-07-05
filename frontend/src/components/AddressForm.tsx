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
        setResponse('Address updated successfully');
      } else {
        address = await addAddress(repairShopId, street, city, state, zip, country);
        setResponse('Address added successfully');
      }
      onAddressAddedOrUpdated(address);
      setStreet('');
      setCity('');
      setState('');
      setZip('');
      setCountry('');
    } catch (error) {
      setResponse('Error: Unable to save address');
    }
  };

  return (
    <Container>
      <Typography variant="h6">{addressId ? 'Update Address' : 'Add Address'}</Typography>
      <TextField
        label="Street"
        value={street}
        onChange={(e) => setStreet(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="City"
        value={city}
        onChange={(e) => setCity(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="State"
        value={state}
        onChange={(e) => setState(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Zip"
        value={zip}
        onChange={(e) => setZip(e.target.value)}
        fullWidth
        margin="normal"
      />
      <TextField
        label="Country"
        value={country}
        onChange={(e) => setCountry(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleSaveAddress}>
        {addressId ? 'Update Address' : 'Add Address'}
      </Button>
      {response && <Typography>{response}</Typography>}
    </Container>
  );
};

export default AddressForm;

