import { useEffect, useRef, useState } from 'react'

interface AnimatedNumberProps {
  value: number
  /** Animation duration in milliseconds. Defaults to 450ms. */
  duration?: number
  /** Number of decimal places to display. Defaults to 0. */
  decimals?: number
}

/**
 * Smoothly tweens a numeric value from its previous state to a new one using
 * requestAnimationFrame and an easeOutCubic curve. Cancels pending animations
 * when the target changes mid-flight.
 */
export function AnimatedNumber({
  value,
  duration = 450,
  decimals = 0,
}: AnimatedNumberProps) {
  const [display, setDisplay] = useState(value)
  const displayRef = useRef(value)

  // Keep ref in sync with latest display so interrupts start from current pos.
  useEffect(() => {
    displayRef.current = display
  }, [display])

  useEffect(() => {
    const from = displayRef.current
    const to = value

    // Skip trivial updates (avoid rAF jitter for unchanged values).
    if (Math.abs(from - to) < 0.0001) {
      setDisplay(to)
      return
    }

    let rafId = 0
    let start: number | null = null

    const step = (ts: number) => {
      if (start === null) start = ts
      const elapsed = ts - start
      const t = Math.min(elapsed / duration, 1)
      const eased = 1 - Math.pow(1 - t, 3) // easeOutCubic
      const next = from + (to - from) * eased
      setDisplay(next)
      if (t < 1) {
        rafId = requestAnimationFrame(step)
      } else {
        setDisplay(to)
      }
    }

    rafId = requestAnimationFrame(step)
    return () => cancelAnimationFrame(rafId)
  }, [value, duration])

  const formatted =
    decimals > 0 ? display.toFixed(decimals) : Math.round(display).toString()
  return <>{formatted}</>
}
