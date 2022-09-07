import { writable } from 'svelte/store'

// seconds
export const remainingTime = writable(0)
export const active = writable(false)
export const paused = writable(false)
export const compleatedCount = writable(0)
export const isBreak = writable(false)

let workTime = 25 * 60
let breakTime = 5 * 60
let longBreakTime = 15 * 60

let countDown: NodeJS.Timer

export function startTimer(duration: number = workTime) {
	active.set(true)
	paused.set(false)
	remainingTime.set(duration)

	countDown = setInterval(() => {
		remainingTime.update(time => {
			if (time === 0 ) {
				let nextTime: number
				compleatedCount.update(count => {
					isBreak.update(isBreak => {
						if (isBreak === false) {
							if (count % 4 === 0) {
								nextTime = longBreakTime
							} else {
								nextTime = breakTime
							}
						} else {
							nextTime = workTime
						}
						return !isBreak
					})
					return count + 1
				})
				return nextTime
			}
			return time - 1
		})
		}, 1000)
}

export function stopTimer() {
	active.set(false)
	paused.set(false)
	clearInterval(countDown)
	remainingTime.set(0)
}
