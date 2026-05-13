import type { CSSProperties } from 'react'

interface SkeletonProps {
  /** Width in pixels or any valid CSS length. Defaults to 100%. */
  width?: number | string
  /** Height in pixels or any valid CSS length. Defaults to 16px. */
  height?: number | string
  /** Border radius. Defaults to 4px. */
  radius?: number | string
  /** Extra class names for layout tweaks. */
  className?: string
  /** Delay in ms before shimmer starts — stagger to avoid pulse sync. */
  delay?: number
}

/**
 * Dependency-free shimmer block. Animates via a moving linear-gradient
 * background. Used by the main app while the first GPU stats are loading.
 */
export function Skeleton({
  width = '100%',
  height = 16,
  radius = 4,
  className = '',
  delay = 0,
}: SkeletonProps) {
  const style: CSSProperties = {
    width: typeof width === 'number' ? `${width}px` : width,
    height: typeof height === 'number' ? `${height}px` : height,
    borderRadius: typeof radius === 'number' ? `${radius}px` : radius,
    animationDelay: `${delay}ms`,
  }
  return <div className={`skeleton ${className}`.trim()} style={style} aria-hidden />
}
