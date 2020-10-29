import React, {useEffect, useState} from 'react';
import {Button, Card, Divider, Dropdown, Form, Grid, Input, Statistic, Label, Segment} from 'semantic-ui-react';

import {useSubstrate} from './substrate-lib';
import BurgerkingContract, {defaultGasLimit} from "./BurgerkingContract";


function Main(props) {
    const {api, keyring} = useSubstrate();
    const {accountPair} = props;
    const burgerkingContract = BurgerkingContract(api);

    const keyringOptions = keyring.getPairs().map(account => ({
        key: account.address,
        value: account.address,
        text: account.meta.name.toUpperCase(),
        icon: 'user'
    }));

    const [totalSupply, setTotalSupply] = useState(0);
    const [balance, setBalance] = useState(0);
    const [formState, setFormState] = useState({addressTo: null, amount: 0});
    const onChange = (_, data) =>
        setFormState(prev => ({...prev, [data.state]: data.value}));

    const {addressTo, amount} = formState;

    const onSelectAddressTo = address => setFormState(prev => ({...prev, 'addressTo': address}));

    const transfer = () => {
        burgerkingContract.tx.transfer(0, defaultGasLimit, addressTo, amount).signAndSend(accountPair, (result) => {
            updateBalance();
        });
    }

    const updateBalance = () => {
        burgerkingContract.query.balanceOf(accountPair.address, 0, defaultGasLimit, accountPair.address).then((balance) => {
            setBalance(balance.output.toNumber());
        })
    }
    useEffect(() => {
        let unsubscribe;
        burgerkingContract.query.totalSupply(keyring.getPairs()[0].address, 0, defaultGasLimit).then((total) => {
            console.log('total output' + total.output);
            setTotalSupply(total.output.toNumber());
            updateBalance();
        }).then(unsub => {
            unsubscribe = unsub;
        }).catch(console.error);
        return () => unsubscribe && unsubscribe();
    }, [accountPair]);

    return (
        <Grid.Column>
            <Label>
            <h1>BURGER TOKEN</h1>
            </Label>
            <Card.Group>
                <Card>
                <Segment color='grey' inverted>
                    <Statistic value={totalSupply} label={'BURGER SUPLY'}/>
                    </Segment>
                </Card>
                <Card>
                <Segment color='grey' inverted>
                    <Statistic value={balance} label={'Your BURGER '}/>
                    </Segment>
                </Card>
            </Card.Group>
            <Divider hidden/>
            <Form>
                <Form.Group inline>
                    <Form.Field>
                    <Label pointing="right">Select an Account</Label>
                        <Dropdown
                            search
                            selection
                            clearable
                            placeholder='Select an account -ex BOB Account'
                            options={keyringOptions}
                            onChange={(_, dropdown) => {
                                onSelectAddressTo(dropdown.value);
                            }}
                        />
                    </Form.Field>
                    <Form.Field width={4}>
                        <Input
                            fluid
                            label='Amount'
                            type='number'
                            state='amount'
                            onChange={onChange}
                        />
                    </Form.Field>
                    <Form.Field style={{textAlign: 'center'}}>
                        <Button onClick={transfer}>Transfer</Button>
                    </Form.Field>
                </Form.Group>
            </Form>
        </Grid.Column>
    );
}

export default function burgerking_do_tf(props) {
    const {api} = useSubstrate();
    const {accountPair} = props;
    return (api.registry && accountPair
        ? <Main {...props} /> : null);
}
