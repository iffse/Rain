import { writable } from 'svelte/store'
import { sendNotification } from '@tauri-apps/api/notification'

// seconds
let workTime = 25 * 60
let breakTime = 5 * 60
let longBreakTime = 15 * 60

export const remainingTime = writable(workTime)
export const active = writable(false)
export const paused = writable(false)
export const compleatedCount = writable(0)
export const isBreak = writable(false)

let countDown: NodeJS.Timer

export function startTimer() {
	active.set(true)
	paused.set(false)

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
							sendNotification({
								title: 'Break Time',
								body: 'Take a break',
							})
						} else {
							nextTime = workTime
							sendNotification({
								title: 'Work Time',
								body: 'Get back to work',
							})
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
	isBreak.update(isBreak => {
		if (isBreak) {
			compleatedCount.update(count => {
				if (count % 4 === 0) {
					remainingTime.set(longBreakTime)
				} else {
					remainingTime.set(breakTime)
				}
				return count
			})
		} else {
			remainingTime.set(workTime)
		}
		return isBreak
	})
	remainingTime.set(workTime)
}

export function pauseTimer() {
	paused.set(true)
	clearInterval(countDown)
}
