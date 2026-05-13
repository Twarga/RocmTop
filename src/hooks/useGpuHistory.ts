import { useEffect, useRef, useState } from 'react'
import type { GpuStats } from '../types/gpu'

export interface GpuHistory {
  temperature: number[]
  gpuClock: number[]
  gpuBusy: number[]
  vramUsedMb: number[]
}

const EMPTY_HISTORY: GpuHistory = {
  temperature: [],
  gpuClock: [],
  gpuBusy: [],
  vramUsedMb: [],
}

/**
 * Maintain a rolling window of GPU samples, one entry per poll.
 * Default 30 samples × 2s polling = 60s of history.
 *
 * Why not useState alone: batch copies would create a new array every tick.
 * Fine for 30 × 4 numbers; we favour clarity over squeezing allocations.
 */
export function useGpuHistory(stats: GpuStats | null, capacity = 30): GpuHistory {
  const [history, setHistory] = useState<GpuHistory>(EMPTY_HISTORY)
  const lastSeen = useRef<GpuStats | null>(null)

  useEffect(() => {
    if (!stats || stats === lastSeen.current) return
    lastSeen.current = stats

    setHistory((prev) => ({
      temperature: appendCapped(prev.temperature, stats.temperature, capacity),
      gpuClock: appendCapped(prev.gpuClock, stats.gpu_clock, capacity),
      gpuBusy: appendCapped(prev.gpuBusy, stats.gpu_busy, capacity),
      vramUsedMb: appendCapped(
        prev.vramUsedMb,
        Math.round(stats.vram_used / 1048576),
        capacity,
      ),
    }))
  }, [stats, capacity])

  return history
}

function appendCapped(buf: number[], next: number, capacity: number): number[] {
  if (buf.length < capacity) return [...buf, next]
  return [...buf.slice(buf.length - capacity + 1), next]
}
