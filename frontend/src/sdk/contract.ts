import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import * as abi_router from './abi/router_contract.json';
import { Signer } from '@polkadot/api/types';
import { IKeyringPair } from '@polkadot/types/types';
import { BN } from '@polkadot/util';
import {
    contractTx,
  } from '@scio-labs/use-inkathon'

class RouterClient {
    api: ApiPromise;
    contract: ContractPromise;

    constructor(api: ApiPromise, router_address: string) {
        this.api = api;
        this.contract = new ContractPromise(api, abi_router, router_address);
    }

    async swap_exact_tokens_for_tokens(
        sender: IKeyringPair,
        from_asset: string,
        to_asset: string,
        amount: BN,
        min_expected: BN
    ) {
        return contractTx(
            this.api,
            sender,
            this.contract,
            "swap_exact_tokens_for_tokens",
            undefined,
            [amount, min_expected, [from_asset, to_asset], sender.address, Date.now() + 1e6]
        )
    }
}
