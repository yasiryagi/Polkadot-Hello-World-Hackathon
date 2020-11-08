import { Abi, ContractPromise } from '@polkadot/api-contract';
import contractMetadata from './burgermetadata.json';

const abi = new Abi(contractMetadata);
const addr = '5CKG1XnUATa2dwTFZY3MUEN53nRaJWoGKR7vpT7x1cfb5dJv';

export default function BurgerMetaContract (api) {
  return new ContractPromise(api, abi, addr);
}
