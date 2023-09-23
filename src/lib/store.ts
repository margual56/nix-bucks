import { type Writable, writable } from 'svelte/store';

export interface Subscription {
    uuid: string,
    name: string,
    cost: number,
    recurrence: string,
};

export interface Punctual {
    uuid: string,
    name: string,
    cost: number,
    date: string,
};

export const subscriptions: Writable<Subscription[]> = writable([]);
export const incomes: Writable<Subscription[]> = writable([]);
export const p_incomes: Writable<Punctual[]> = writable([]);
export const p_expenses: Writable<Punctual[]> = writable([]);

export const monthly_cost: Writable<string> = writable("Loading...");
export const eoy_cost: Writable<string> = writable("Loading...");
export const eoy_income: Writable<string> = writable("Loading...");
export const eoy_balance: Writable<string> = writable("Loading...");
export const eom_balance: Writable<string> = writable("Loading...");
