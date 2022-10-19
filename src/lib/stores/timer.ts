import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api'

// let startTime = await invoke('get_timer_parameters')


export const remainingTime = writable(0)
export const active = writable(false)
export const paused = writable(false)
export const compleatedCount = writable(0)
export const isBreak = writable(false)

update_parameters()

let countDown: NodeJS.Timer
let update = false

export function startTimer() {
	active.set(true)
	paused.set(false)

	countDown = setInterval(async () => {
		// let remaining: number = await invoke('update_timer')
		let remaining: number = await invoke('timer_update')
		if (update) {
			update_parameters()
			update = false
		}
		if (remaining <= 0) {
			update = true
		}
		remainingTime.set(remaining)
		}, 1000)
}

export async function stopTimer() {
	active.set(false)
	paused.set(false)
	clearInterval(countDown)
	remainingTime.set(await invoke('timer_reset'))
}

export function pauseTimer() {
	paused.set(true)
	clearInterval(countDown)
}

export async function skipTimer() {
	await invoke('timer_skip')
	update_parameters()
}

async function update_parameters() {
	let parameters = await invoke('timer_get_parameters')
	remainingTime.set(parameters[0])
	isBreak.set(parameters[1])
	compleatedCount.set(parameters[2])
}
