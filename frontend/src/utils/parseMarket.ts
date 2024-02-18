import { sha256AsU8a } from "@polkadot/util-crypto";
import { Market } from "~/types";
import { toHex } from "./encoding";

export function parseMarket(
  jsonMarket: any,
  id: number,
  rawMarket: any,
): Market {
  const tokenA = Number(rawMarket.balanceA.replaceAll(',', ''));
  const tokenB = Number(rawMarket.balanceB.replaceAll(',', ''));
  console.log(tokenA, tokenB)
  const tokensSum = tokenA + tokenB;

  return {
    id,
    title: jsonMarket.title,
    description: jsonMarket.description,
    shortPct: 100 - (tokenA / tokensSum) * 100,
    longPct: 100 - (tokenB / tokensSum) * 100,
    expireDate: new Date(
      Number(rawMarket.market.expiredAt.replaceAll(",", "")),
    ),
  };
}

export function getMarketHash(jsonMarket: any) {
  return toHex(sha256AsU8a(JSON.stringify(jsonMarket)));
}
