import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  // The currently stored value
  const [currentValue, setCurrentValue] = useState(0);
  const [formValue, setFormValue] = useState(0);
  const [currentName, setCurrentName] = useState("");
  const [currentNumber, setCurrentNumber]=useState(0);
  const [Name, setName] = useState("");
  const [Number, setNumber] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.templateModule.people(newValue => {
      // The storage value is an Option<u32>
      // So we have to check whether it is None first
      // There is also unwrapOr
      if (newValue.isNone) {
        setCurrentName("Default");
        setCurrentNumber(0);
      } else {
        setCurrentName(newValue.name.toHuman());
        setCurrentNumber(newValue.number);
      }
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [api.query.templateModule]);

  return (
    <Grid.Column width={8}>
      <h1>Proflie</h1>
      <Card centered>
        <Card.Content >
          <Card.Header content={'User Name :  '+currentName}/>
          <Card.Meta content={'ID Number   '+currentNumber} />
        </Card.Content>
      </Card>
      <Form>
        <Form.Field>
          <Input
            label='Name'
            state='newValue'
            type='string'
            onChange={(_, { value }) => setName(value)}
          />
        </Form.Field>
        <Form.Field>
          <Input
            label='Id Number'
            state='newValue'
            type='number'
            onChange={(_, { value }) => setNumber(value)}
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Update Name'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'templateModule',
              callable: 'changeName',
              inputParams: [Name ],
              paramFields: [true]
            }}
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Update Number'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'templateModule',
              callable: 'changedNumber',
              inputParams: [Number],
              paramFields: [true]
            }}
          />
        </Form.Field>

        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  );
}

export default function TemplateModule (props) {
  const { api } = useSubstrate();
  return (api.query.templateModule && api.query.templateModule.something
    ? <Main {...props} /> : null);
}
