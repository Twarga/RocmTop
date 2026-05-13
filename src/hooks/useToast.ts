import { useCallback, useEffect, useState } from 'react'

export type ToastVariant = 'info' | 'success' | 'error'

export interface ToastState {
  id: number
  message: string
  variant: ToastVariant
}

/**
 * Minimal toast hook — one active toast at a time, auto-dismisses.
 * Returns the current toast (or null) and a `show` function.
 */
export function useToast(durationMs = 3200) {
  const [toast, setToast] = useState<ToastState | null>(null)

  const show = useCallback((message: string, variant: ToastVariant = 'info') => {
    setToast({ id: Date.now() + Math.random(), message, variant })
  }, [])

  const dismiss = useCallback(() => setToast(null), [])

  useEffect(() => {
    if (!toast) return
    const timer = setTimeout(() => setToast(null), durationMs)
    return () => clearTimeout(timer)
  }, [toast, durationMs])

  return { toast, show, dismiss }
}
