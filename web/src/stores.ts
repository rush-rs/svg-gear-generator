import { writable } from 'svelte/store'

export enum SchemeKind {
    Light,
    Dark,
    System,
}
export let colorScheme = writable(SchemeKind.System)
export let darkTheme = writable(false)
