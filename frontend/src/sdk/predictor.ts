import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import * as abi_predicotor from './abi/predictor.json';
import { Signer } from '@polkadot/api/types';
import { IKeyringPair } from '@polkadot/types/types';
import { BN } from '@polkadot/util';
import {
    contractTx,
  } from '@scio-labs/use-inkathon'

class PredictorClient {
    api: ApiPromise;
    contract: ContractPromise;

    constructor(api: ApiPromise, contract_address: string) {
        this.api = api;
        this.contract = new ContractPromise(api, abi_predicotor, contract_address);
    }

    async mint(
        sender: IKeyringPair,
        market_id: Number,
        amount: BN,
    ) {
        return contractTx(
            this.api,
            sender,
            this.contract,
            "mint",
            undefined,
            [market_id, amount]
        )
    }
}
