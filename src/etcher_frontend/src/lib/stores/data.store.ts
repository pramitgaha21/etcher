import { writable } from "svelte/store";

export type BlockId = bigint | null;

export const blockId = writable<BlockId>(null);
