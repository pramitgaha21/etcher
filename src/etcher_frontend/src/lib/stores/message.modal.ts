import { writable } from "svelte/store"

export type Message = {
        show: boolean,
        messageTitle: string,
        message: string
}

export const message = writable<Message>({
        show: false,
        messageTitle: "",
        message: "",
})
