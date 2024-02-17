import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import * as abi_router from './abi/router_contract.json';
import { Signer } from '@polkadot/api/types';
import { IKeyringPair } from '@polkadot/types/types';
import { BN } from '@polkadot/util';
import {
    contractTx,
    contractQuery,
  } from './interact'


export class RouterClient {
    api: ApiPromise;
    contract: ContractPromise;

    constructor(api: ApiPromise, router_address: string) {
        this.api = api;
        this.contract = new ContractPromise(api, abi_router, router_address);
    }

    calc_min_rcv(amount: BN) {
        return amount.mul(new BN(95)).div(new BN(100));
    }

    async get_amounts_out(
        sender: string,
        amount_in: BN,
        path: string[],
    ) {
        return contractQuery(
            this.api,
            sender,
            this.contract,
            "Router::get_amounts_out",
            undefined,
            [amount_in, path]
        )
    }

    async swap_exact_tokens_for_tokens(
        sender: string,
        signer: Signer,
        amount_in: BN,
        amount_out_min: BN,
        path: string[],
    ) {
        return contractTx(
            this.api,
            sender,
            signer,
            this.contract,
            "Router::swap_exact_tokens_for_tokens",
            undefined,
            [amount_in, amount_out_min, path, sender, Date.now() + 1e6]
        )
    }

    async add_liquidity(
        sender: string,
        signer: Signer,
        asset_a: string,
        asset_b: string,
        amount_a: BN,
        amount_b: BN,
    ) {
        return contractTx(
            this.api,
            sender,
            signer,
            this.contract,
            "Router::add_liquidity",
            undefined,
            [asset_a, asset_b, amount_a, amount_b, this.calc_min_rcv(amount_a), this.calc_min_rcv(amount_b), sender, Date.now() + 1e6]
        )
    }
}
