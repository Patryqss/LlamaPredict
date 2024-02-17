import type { EventType } from "mitt";

export interface EmitterEvents extends Record<EventType, any> {
  "select-account": undefined;
  "txn-success": string;
}
