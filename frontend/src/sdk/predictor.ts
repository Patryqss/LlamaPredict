import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import { Hash } from '@polkadot/types/interfaces';
import * as abi_predicotor from './abi/predictor.json';
import { Signer } from '@polkadot/api/types';
import { IKeyringPair } from '@polkadot/types/types';
import { BN } from '@polkadot/util';
import {
    contractTx,
  } from './interact'

export class PredictorClient {
    api: ApiPromise;
    contract: ContractPromise;

    constructor(api: ApiPromise, contract_address: string) {
        this.api = api;
        this.contract = new ContractPromise(api, abi_predicotor, contract_address);
    }

    async mint(
        sender: string,
        signer: Signer,
        market_id: Number,
        amount: BN,
    ) {
        return contractTx(
            this.api,
            sender,
            signer,
            this.contract,
            "mint",
            undefined,
            [market_id, amount]
        )
    }

    async add_market(
        sender: string,
        signer: Signer,
        underlying_token: string,
        hash: Hash,
        resolved_at: Number,
        resolution_time: Number,
        collateral_rate: Number,
    ) {
        return contractTx(
            this.api,
            sender,
            signer,
            this.contract,
            "add_market",
            undefined,
            [underlying_token, hash, resolved_at, resolution_time, collateral_rate]
        )
    }
}
