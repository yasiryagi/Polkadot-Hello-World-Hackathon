import React, {useState } from 'react';
import { Form, Grid, Label, Table, Icon, Message } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';

function Main(props) {
  const { api } = useSubstrate();
  const [blockInfo, setBlockInfo] = useState();
  const [blockhash, setBlockhash] = useState();

  const getBlockInfo = async (blockhash) => {
    try {
      const blockInfo = await api.rpc.chain.getHeader(blockhash);

      setBlockInfo(blockInfo);
    } catch (e) {
      console.error(e);
    }
  };


  return (
    <Grid.Column>
      <Message info>
        <Message.Header>FIND YOUR BLOCK HERE</Message.Header>
      </Message>
      <Form
        onSubmit={async (e, { value }) => await getBlockInfo(blockhash)}
        size='small'
      >
        <Form.Group widths={12}>
          <Form.Input
            size='large'
            width={10}
            placeholder={'Paste Hash / Parent hash number here to find the detail'}
            onChange={(e, { value }) => setBlockhash(value)}
          />
          {blockhash && <Form.Button content={<Icon name='search'/>} />}
        </Form.Group>
      </Form>
      {blockInfo && blockInfo.number && (
        <Table celled>
          <Table.Header>
            <Table.Row>
              <Table.HeaderCell>Item</Table.HeaderCell>
              <Table.HeaderCell>Data</Table.HeaderCell>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            <Table.Row>
              <Table.Cell>
                <Label ribbon color='orange'>Block</Label>
              </Table.Cell>
              <Table.Cell>{blockInfo.number.toNumber()}</Table.Cell>
            </Table.Row>
            <Table.Row>
              <Table.Cell>Parent Hash</Table.Cell>
              <Table.Cell>{blockInfo.parentHash.toHuman()}</Table.Cell>
            </Table.Row>
            <Table.Row>
              <Table.Cell>State Root</Table.Cell>
              <Table.Cell>{blockInfo.stateRoot.toHuman()}</Table.Cell>
            </Table.Row>
            <Table.Row>
              <Table.Cell>Extrinsics Root</Table.Cell>
              <Table.Cell>{blockInfo.extrinsicsRoot.toHuman()}</Table.Cell>
            </Table.Row>
          </Table.Body>
        </Table>
      )}
    </Grid.Column>
  );
}

export default function BlockInfoBy(props) {
  const { api } = useSubstrate();
  return api.rpc && api.rpc.chain.getHeader ? <Main {...props} /> : null;
}
