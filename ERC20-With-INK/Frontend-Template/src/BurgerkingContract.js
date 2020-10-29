import metadata from './Burgerkingmeta.json';
import {Abi, ContractPromise} from '@polkadot/api-contract';

export const defaultGasLimit = 300000n * 1000000n;
const BurgerkingContractAdd = '5FY8nR9dDmujyW7iUZ8cv9Kni1ePBA5jAeFBM9YjWoAJGoaF';

export default function BurgerkingContract(api) {
    const abi = new Abi(metadata);
    return new ContractPromise(api, abi, BurgerkingContractAdd);
}

export function display(balance) {
    return balance.toString() + ' BURGER';
}
