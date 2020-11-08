import React  from 'react';

import {
  Menu,
  Container,
  Image,
  Label
} from 'semantic-ui-react';

export default function SelectMove() {
  return (
    <Menu attached='top' tabular
      style={{
        backgroundColor: '#fff',
        borderColor: '#fff',
        paddingTop: '1em',
        paddingBottom: '1em', }}>
      <Container>
        <Menu.Menu>
          <Image
            src={`${process.env.PUBLIC_URL}/assets/substrate-logo.png`}
            size='mini' />
        </Menu.Menu>
        <Menu.Menu position='right' style={{ alignItems: 'center' }}>
      <Label as='a' color='blue' 
      href ='https://github.com/burgerking12/Polkadot-Hello-World-Hackathon/tree/main/ERC20-With-INK'
      target='_blank' >
         <Image
            src={`${process.env.PUBLIC_URL}/assets/veronika.jpg`}
            size='mini' floated='left'/> Burgerking12
    </Label>
        </Menu.Menu>
      </Container>
    </Menu>
  );
}
