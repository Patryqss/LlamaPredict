import type { EventType } from "mitt";

export interface EmitterEvents extends Record<EventType, any> {
  "select-account": undefined;
  "txn-success": string;
}

export type Market = {
  id: number;
  title: string;
  description: string;
  shortPct: number;
  longPct: number;
  expireDate: Date;
};
