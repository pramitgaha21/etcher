import { Actor, HttpAgent } from "@dfinity/agent";

// Imports and re-exports candid interface
import { idlFactory } from "./etcher_backend.did.js";
export { idlFactory } from "./etcher_backend.did.js";

/* CANISTER_ID is replaced by webpack based on node environment
 * Note: canister environment variable will be standardized as
 * process.env.CANISTER_ID_<CANISTER_NAME_UPPERCASE>
 * beginning in dfx 0.15.0
 */
export const canisterId = 'pvxnv-ciaaa-aaaag-qjunq-cai'

export const createActor = (canisterId, options = {}) => {
        const agent = options.agent || new HttpAgent({ ...options.agentOptions });

        if (options.agent && options.agentOptions) {
                console.warn(
                        "Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent."
                );
        }

        // Creates an actor with using the candid interface and the HttpAgent
        return Actor.createActor(idlFactory, {
                agent,
                canisterId,
                ...options.actorOptions,
        });
};
