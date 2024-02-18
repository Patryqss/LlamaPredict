import { ContractPromise } from "@polkadot/api-contract";
import { ApiPromise } from "@polkadot/api";
import * as minty_psp from "./abi/minty_psp22.json";
import * as conditional_psp from "./abi/conditional_psp22.json";
import { Signer } from "@polkadot/api/types";
import { BN } from "@polkadot/util";
import { contractTx, contractQuery, wrapDecodeError, decodeOutput } from "./interact";
import { process_number } from "./utils";

export class PSP22Client {
  api: ApiPromise;
  contract: ContractPromise;

  constructor(
    api: ApiPromise,
    contract_address: string,
    source: Record<string, unknown> = conditional_psp,
  ) {
    this.api = api;
    this.contract = new ContractPromise(api, source, contract_address);
  }

  async balanceOf(user: string): Promise<BN> {
    let r = await contractQuery(
      this.api,
      user,
      this.contract,
      "PSP22::balance_of",
      undefined,
      [user],
    );
    return process_number(wrapDecodeError(decodeOutput(r, this.contract, "PSP22::balance_of")));
  }

  async totalSupply(): Promise<BN> {
    let r = await contractQuery(
      this.api,
      "",
      this.contract,
      "PSP22::total_supply",
      undefined,
      [],
    );
    return process_number(wrapDecodeError(decodeOutput(r, this.contract, "PSP22::total_supply")));
  }

  async increaseAllowance(sender: string, signer: Signer, spender: string, amount: BN) {
    return contractTx(
      this.api,
      sender,
      signer,
      this.contract,
      "PSP22::increase_allowance",
      undefined,
      [spender, amount]
    );
  }
}

export class USDClient extends PSP22Client {
  constructor(api: ApiPromise, contract_address: string) {
    super(api, contract_address, minty_psp);
  }

  async mint(sender: string, signer: Signer, amount: BN) {
    return contractTx(
      this.api,
      sender,
      signer,
      this.contract,
      "PSP22Mintable::mint",
      undefined,
      [amount],
    );
  }
}
