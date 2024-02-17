import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import * as minty_psp from './abi/minty_psp22.json';
import * as conditional_psp from './abi/conditional_psp22.json';
import { Signer } from '@polkadot/api/types';
import { IKeyringPair } from '@polkadot/types/types';
import { BN } from '@polkadot/util';
import {
    contractTx,
    contractQuery
  } from '@scio-labs/use-inkathon'

export class PSP22Client {
    api: ApiPromise;
    contract: ContractPromise;

    constructor(api: ApiPromise, contract_address: string) {
        this.api = api;
        this.contract = new ContractPromise(api, conditional_psp, contract_address);
    }

    async balanceOf(
        address: string
    ) {
        return contractQuery(
            this.api,
            address,
            this.contract,
            "balanceOf",
            undefined,
            [address]
        )
    }
}

export class USDClient extends PSP22Client {
    constructor(api: ApiPromise, contract_address: string) {
        super(api, contract_address);

        this.contract = new ContractPromise(api, minty_psp, contract_address);
    }

    async mint(
        sender: IKeyringPair,
        amount: BN,
    ) {
        return contractTx(
            this.api,
            sender,
            this.contract,
            "mint",
            undefined,
            [amount]
        )
    }
}