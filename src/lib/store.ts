import { type Writable, writable } from 'svelte/store';

export interface Subscription {
    uuid: string,
    name: string,
    cost: number,
    recurrence: string,
};

export const incomes: Writable<Subscription[]> = writable([]);
