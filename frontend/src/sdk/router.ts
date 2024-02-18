import { ContractPromise } from "@polkadot/api-contract";
import { ApiPromise } from "@polkadot/api";
import * as abi_router from "./abi/router_contract.json";
import { Signer } from "@polkadot/api/types";
import { BN } from "@polkadot/util";
import {
  contractTx,
  contractQuery,
  decodeOutput,
  wrapDecodeError,
} from "./interact";
import { PSP22Client } from "./psp22";
import { process_number } from "./utils";
import { FactoryClient } from "./factory";

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

  calc_price_impact_pct(
    amount_in: BN,
    amount_out: BN,
    this_amount: BN,
    other_amount: BN,
  ) {
    let expected = amount_in.mul(other_amount).div(this_amount);
    return (
      ((expected.toNumber() - amount_out.toNumber()) / expected.toNumber()) *
      100
    );
  }

  async get_position_value(user: string, token_a: string, token_b: string) {
    // TODO: specify type
    let factory = await this.get_factory();
    let token_lp = await factory.get_pair(token_a, token_b);
    let client_a = new PSP22Client(this.api, token_a);
    let client_b = new PSP22Client(this.api, token_b);
    let client_lp = new PSP22Client(this.api, token_lp);

    let [total_a, total_b] = await this.get_reserves(token_a, token_b);
    let total_lp = await client_lp.totalSupply();
    let balance_a = await client_a.balanceOf(user);
    let balance_b = await client_b.balanceOf(user);
    let balance_lp = await client_lp.balanceOf(user);
    const balance_a_cache = balance_a.clone();
    const balance_b_cache = balance_b.clone();

    balance_a = balance_a.add(balance_lp.mul(total_a).div(total_lp));
    balance_b = balance_b.add(balance_lp.mul(total_b).div(total_lp));

    let dolla: BN;
    // dolla is minimum of balance_a and balance_b
    if (balance_a.cmp(balance_b) == -1) {
      dolla = balance_a.clone();
    } else {
      dolla = balance_b.clone();
    }
    balance_a = balance_a.sub(dolla);
    balance_b = balance_b.sub(dolla);

    if (total_a.cmp(total_b) == -1) {
      dolla = dolla.add(
        balance_a.muln(
          1 - total_a.toNumber() / (total_a.toNumber() + total_b.toNumber()),
        ),
      );
      dolla = dolla.add(
        balance_b.muln(
          total_a.toNumber() / (total_a.toNumber() + total_b.toNumber()),
        ),
      );
    } else {
      dolla = dolla.add(
        balance_b.muln(
          1 - total_b.toNumber() / (total_a.toNumber() + total_b.toNumber()),
        ),
      );
      dolla = dolla.add(
        balance_a.muln(
          total_b.toNumber() / (total_a.toNumber() + total_b.toNumber()),
        ),
      );
    }

    return {
      balance_a: balance_a_cache,
      balance_b: balance_b_cache,
      position_value: dolla,
    };
  }

  calculate_close_position(
    user_a: number,
    user_b: number,
    total_a: number,
    total_b: number,
  ) {
    let sell_outcome: string;
    if (user_a > user_b) {
      sell_outcome = "A";
    } else if (user_a < user_b) {
      sell_outcome = "B";
      [total_a, total_b] = [total_b, total_a];
      [user_a, user_b] = [user_b, user_a];
    } else {
      return { sell_outcome: "NONE", to_swap: 0 };
    }

    // now a > b, because of the whack swap we did above
    let a = user_a - user_b; // ease calculation by assuming b = 0
    let A = total_a;
    let B = total_b;
    let root_delta = Math.sqrt((a - A - B) ** 2 + 4 * a * A);
    let to_swap = root_delta + a - A - B;

    return {
      sell_outcome,
      to_swap,
    };
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
      [asset_a, asset_b],
    );
    return wrapDecodeError(decodeOutput(r, this.contract, "get_pair"));
  }

  async get_factory(): Promise<FactoryClient> {
    let r = await contractQuery(
      this.api,
      "",
      this.contract,
      "Router::factory",
      undefined,
      [],
    );
    let addr = wrapDecodeError(
      decodeOutput(r, this.contract, "Router::factory"),
    );
    return new FactoryClient(this.api, addr);
  }

  async get_reserves(asset_a: string, asset_b: string): Promise<Array<BN>> {
    let r = await contractQuery(
      this.api,
      "",
      this.contract,
      "Router::get_reserves",
      undefined,
      [asset_a, asset_b],
    );
    let [a, b] = wrapDecodeError(
      decodeOutput(r, this.contract, "Router::get_reserves"),
    ).Ok;
    return [process_number(a), process_number(b)];
  }

  async get_amount_out(
    amount_in: BN,
    this_amount: BN,
    other_amount: BN,
  ): Promise<BN> {
    let r = await contractQuery(
      this.api,
      "",
      this.contract,
      "Router::get_amount_out",
      undefined,
      [amount_in, this_amount, other_amount],
    );
    return process_number(
      wrapDecodeError(decodeOutput(r, this.contract, "Router::get_amount_out"))
        .Ok,
    );
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
      [amount_in, amount_out_min, path, sender, Date.now() + 1e6],
    );
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
      [
        asset_a,
        asset_b,
        amount_a,
        amount_b,
        new BN(0),
        new BN(0),
        sender,
        Date.now() + 1e6,
      ],
    );
  }
}
