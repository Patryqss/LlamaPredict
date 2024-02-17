import { sha256AsU8a } from "@polkadot/util-crypto";
import { Market } from "~/types";
import { toHex } from "./encoding";

export function parseMarket(
  jsonMarket: any,
  id: number,
  rawMarket: any,
): Market {
  const tokensSum = rawMarket.balanceA + rawMarket.balanceB;

  return {
    id,
    title: jsonMarket.title,
    description: jsonMarket.description,
    shortPct: 100 - (rawMarket.balanceA / tokensSum) * 100,
    longPct: 100 - (rawMarket.balanceB / tokensSum) * 100,
    expireDate: new Date(
      Number(rawMarket.market.expiredAt.replaceAll(",", "")),
    ),
  };
}

export function getMarketHash(jsonMarket: any) {
  return toHex(sha256AsU8a(JSON.stringify(jsonMarket)));
}
