import { writable } from "svelte/store";

export const btcDepositAddress = writable("");

export const ckbtcDepositAddress = writable("");

export type BlockId = number | null;

export const blockId = writable<BlockId>(null);