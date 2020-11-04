import React, { useState } from 'react';
import { Form, Input, Grid, Label, Icon, Message } from 'semantic-ui-react';
import { TxButton } from './substrate-lib/components';
import { useSubstrate } from './substrate-lib';

export default function Main (props) {
  const [status, setStatus] = useState(null);
  const [formState, setFormState] = useState({ threshold: 0,
    otherSignatories: '', addressTo: null, amount: 0 });
  const { accountPair } = props;
  const onChange = (_, data) =>
    setFormState(prev => ({ ...prev, [data.state]: data.value }));
  const { threshold, otherSignatories, addressTo, amount } = formState;

  const { api } = useSubstrate();

  return (
    <Grid.Column width={8} style={{ textAlign: 'center'}}>
      <h1>Multisig Transfer</h1>
      <Message size='mini' color='red'> <span role="img" aria-label="hourglass">⌛</span>DON'T USE ALICE ADDRESS [ BASE ] AS A SIGNATORIES</Message>
      <Form>
        <Form.Field>
          <Label basic color='blue' pointing = 'left'>
            <Icon name='fork' content='Fork'/>
            {/* <Label pointing = 'left' basic = 'true' color ='blue'/> */}
            1 Unit = 1000000000000
          </Label>
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label='Transfer To'
            type='text'
            placeholder='address'
            state='addressTo'
            onChange={onChange} />
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label='Amount'
            type='number'
            state='amount'
            onChange={onChange} />
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label='Threshold'
            type='number'
            placeholder='Max as 10'
            state='threshold'
            onChange={onChange} />
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label ='Signatories'
            type='text'
            placeholder='Addresses, separated by comma'
            state='otherSignatories'
            onChange={onChange}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Submit'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'multisig',
              callable: 'asMulti',
              inputParams: [threshold, otherSignatories.split(','), null,
                api.tx.balances.transfer(addressTo, amount), false, 10000000000],
              paramFields: [true, true, { optional: true }, true, true, true]
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  );
}
