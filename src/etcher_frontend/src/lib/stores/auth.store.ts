import { InProd } from '$lib';
import type { Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import { writable } from 'svelte/store';

type OptionalIdentity = null | Identity;

export const identityStore = writable<OptionalIdentity>(null);

const identityProviderUrl = InProd
        ? 'https://identity.ic0.app/'
        : `http://${'rdmx6-jaaaa-aaaaa-aaadq-cai'}.localhost:4943`;


export const connectII = async () => {
        const authClient = await AuthClient.create();

        const AUTH_MAX_TIME_TO_LIVE = BigInt(60 * 60 * 1000 * 1000 * 1000);

        return new Promise((resolve, reject) => {
                authClient.login({
                        identityProvider: identityProviderUrl,
                        maxTimeToLive: AUTH_MAX_TIME_TO_LIVE,
                        onSuccess: () => {
                                identityStore.set(authClient.getIdentity());
                                resolve();
                        },
                        onFailure: (error) => {
                                reject(error);
                        },
                });
        });
};
