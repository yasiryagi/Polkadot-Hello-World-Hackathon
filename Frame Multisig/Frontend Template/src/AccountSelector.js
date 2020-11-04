import React, { useState, useEffect } from 'react';
import { CopyToClipboard } from 'react-copy-to-clipboard';

import {
  Menu,
  Button,
  Dropdown,
  Container,
  Icon,
  Image,
  Label
} from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';

function Main (props) {
  const { keyring } = useSubstrate();
  const { setAccountAddress } = props;
  const [accountSelected, setAccountSelected] = useState('');

  // Get the list of accounts we possess the private key for
  const keyringOptions = keyring.getPairs().map((account) => ({
    key: account.address,
    value: account.address,
    text: account.meta.name.toUpperCase(),
    icon: 'user',
  }));

  const initialAddress =
    keyringOptions.length > 0 ? keyringOptions[0].value : '';

  // Set the initial address
  useEffect(() => {
    setAccountAddress(initialAddress);
    setAccountSelected(initialAddress);
  }, [setAccountAddress, initialAddress]);

  const onChange = (address) => {
    // Update state with new account address
    setAccountAddress(address);
    setAccountSelected(address);
  };

  return (
    <Menu
      attached='top'
      tabular
      style={{
        backgroundColor: '#fff',
        borderColor: '#fff',
        paddingTop: '1em',
        paddingBottom: '1em',
      }}
    >
      <Container>
        <Menu.Menu>
          <Image
            src={`${process.env.PUBLIC_URL}/assets/substrate-logo.png`}
            size='mini'
          />
        </Menu.Menu>
        <Menu.Menu position='right' style={{ alignItems: 'center' }}>
      <Label as='a' color='blue' 
      href ='https://github.com/burgerking12/Frontend-Template-Host'
      target='_blank' >
         <Image
            src={`${process.env.PUBLIC_URL}/assets/veronika.jpg`}
            size='mini' floated='left'
          />
      Burgerking12
    </Label>
        </Menu.Menu>
      </Container>
    </Menu>
  );
}



export default function AccountSelector(props) {
  const { api, keyring } = useSubstrate();
  return keyring.getPairs && api.query ? <Main {...props} /> : null;
}
