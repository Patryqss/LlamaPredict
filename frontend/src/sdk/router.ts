import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import * as abi_router from './abi/router_contract.json';
import { Signer } from '@polkadot/api/types';
import { IKeyringPair } from '@polkadot/types/types';
import { BN } from '@polkadot/util';
import {
    contractTx,
    contractQuery,
    decodeOutput,
    wrapDecodeError,
  } from './interact'
import { PSP22Client } from './psp22';

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

    calc_price_impact_pct(amount_in: BN, amount_out: BN, this_amount: BN, other_amount: BN) {
        let expected = amount_in.mul(other_amount).div(this_amount);
        return (expected.toNumber() - amount_out.toNumber()) / expected.toNumber() * 100;
    }

    async get_position_value(
        user: string,
        token_a: string,
        token_b: string,
    ) {  // TODO: specify type
        let token_lp = await this.get_pair(user, token_a, token_b);
        let client_a = new PSP22Client(this.api, token_a);
        let client_b = new PSP22Client(this.api, token_b);
        let client_lp = new PSP22Client(this.api, token_lp);

        let [total_a, total_b] = await this.get_reserves(user, token_a, token_b);
        let total_lp = await client_lp.totalSupply();
        let balance_a = await client_a.balanceOf(user);
        let balance_b = await client_b.balanceOf(user);
        let balance_lp = await client_lp.balanceOf(user);

        balance_a += balance_lp * total_a / total_lp;
        balance_b += balance_lp * total_b / total_lp;

        let dolla = 0;
        // dolla is minimum of balance_a and balance_b
        if (balance_a < balance_b) {
            dolla = balance_a;
        } else {
            dolla = balance_b;
        }
        balance_a -= dolla;
        balance_b -= dolla;

        if (total_a < total_b) {
            dolla += balance_a * (1 - total_a / (total_a + total_b));
            dolla += balance_b * (total_a / (total_a + total_b));
        } else {
            dolla += balance_b * (1 - total_b / (total_a + total_b));
            dolla += balance_a * (total_b / (total_a + total_b));
        }

        return dolla;
    }

    async get_pair(
        sender: string,
        asset_a: string,
        asset_b: string,
    ): Promise<string> {
        let r = await contractQuery(
            this.api,
            sender,
            this.contract,
            "get_pair",
            undefined,
            [asset_a, asset_b]
        );
        return wrapDecodeError(decodeOutput(r, this.contract, "get_pair"));
    }

    async get_reserves(
        sender: string,
        asset_a: string,
        asset_b: string,
    ){ // TODO: typed return
        let r = await contractQuery(
            this.api,
            sender,
            this.contract,
            "Router::get_reserves",
            undefined,
            [asset_a, asset_b]
        )
        return wrapDecodeError(decodeOutput(r, this.contract, "get_pair"));
    }

    async get_amount_out(
        sender: string,
        amount_in: BN,
        this_amount: BN,
        other_amount: BN,
    ) { // TODO: typed return
        let r = await contractQuery(
            this.api,
            sender,
            this.contract,
            "Router::get_amount_out",
            undefined,
            [amount_in, this_amount, other_amount]
        )
        return wrapDecodeError(decodeOutput(r, this.contract, "get_amount_out"));
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
