import { ContractPromise } from "@polkadot/api-contract";
import { ApiPromise } from "@polkadot/api";
import * as abi_f from "./abi/factory_contract.json";
import { contractQuery, decodeOutput, wrapDecodeError } from "./interact";

export class FactoryClient {
  api: ApiPromise;
  contract: ContractPromise;

  constructor(api: ApiPromise, factory_adddress: string) {
    this.api = api;
    this.contract = new ContractPromise(api, abi_f, factory_adddress);
  }

  async get_pair(token_a: string, token_b: string): Promise<string> {
    let r = await contractQuery(
      this.api,
      "",
      this.contract,
      "Factory::get_pair",
      undefined,
      [token_a, token_b],
    );
    return wrapDecodeError(decodeOutput(r, this.contract, "Factory::get_pair"));
  }
}
