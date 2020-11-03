const { ApiPromise, WsProvider } = require('@polkadot/api');
const chalk = require('chalk');
const log = console.log;
// const ⏪  
 
const PROVIDER = 'wss://kusama-rpc.polkadot.io';
const blockID = process.argv[2];
let do_hash = false;

if (process.argv[2] && process.argv[2].startsWith('0x')) {
  do_hash = true;
}

async function getApi(p) {
  const provider = new WsProvider(p);
  return await ApiPromise.create({ provider });
}

async function displayLatestBlock(url) {
  const api = await getApi(url);
  const block = await api.rpc.chain.getBlock();
  log(chalk.greenBright('LATEST BLOCK ON  ⏩') + `KUSAMA` + chalk.greenBright('⏪') +  `${block} `);
  // log(
  //   chalk.greenBright('DETAIL OF BLOCK ◻ ⏩') + `${blockId}` + chalk.greenBright('⏪') + `is ${url} : ${block}` 
  // );
}

async function displayBlock(blockId, url) {
  const api = await getApi(url);
  let block;
  if (do_hash) {
    block = await api.rpc.chain.getBlock(blockId);
  } else {
    const blockHash = await api.rpc.chain.getBlockHash(blockId);
    block = await api.rpc.chain.getBlock(blockHash);
  }
  log(
    chalk.greenBright('DETAIL BLOCK OF ⏩') + `${blockId}` + chalk.greenBright('⏪') + `is ${url} : ${block}` 
  );
}

async function main() {
  if (blockID) {
    await displayBlock(parseInt(blockID), PROVIDER);
  } else {
    await displayLatestBlock(PROVIDER);
  }
}

main()
  .catch(console.error)
  .finally(() => process.exit());