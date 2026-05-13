import type { ReactNode } from 'react'

interface TooltipProps {
  label: string
  /** Where the bubble points from relative to the wrapped element. */
  placement?: 'top' | 'bottom'
  children: ReactNode
}

/**
 * CSS-only hover tooltip. The `data-tooltip` attribute holds the text, and
 * styles.css renders the bubble via `::after`. Appears after a short delay
 * via transition so quick pointer passes don't flash it.
 */
export function Tooltip({ label, placement = 'top', children }: TooltipProps) {
  return (
    <span className={`tooltip-wrap tooltip-${placement}`} data-tooltip={label}>
      {children}
    </span>
  )
}
