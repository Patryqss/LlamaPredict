import { BN } from "@polkadot/util";

export function process_number(v: string): BN {
    return new BN(v.replaceAll(",", ""));
  }