interface SparklineProps {
  values: number[]
  /** Upper bound for normalization. If omitted or 0, uses max(values). */
  max?: number
  /** Lower bound for normalization. Defaults to 0. */
  min?: number
  /** Stroke/fill accent colour (any CSS colour string). */
  color?: string
  /** Rendered pixel dimensions. */
  width?: number
  height?: number
}

/**
 * Minimal dependency-free SVG sparkline with a filled area beneath the line.
 * Designed for the 60s-history strip under each metric: width stretches to
 * container (viewBox-based), height is fixed.
 */
export function Sparkline({
  values,
  max,
  min = 0,
  color = 'var(--accent-blue)',
  width = 200,
  height = 32,
}: SparklineProps) {
  if (values.length < 2) {
    return (
      <svg
        className="sparkline"
        viewBox={`0 0 ${width} ${height}`}
        preserveAspectRatio="none"
        aria-hidden
      />
    )
  }

  const upper = max && max > 0 ? max : Math.max(...values, 1)
  const range = Math.max(upper - min, 1)

  // Map each sample to a point along the viewBox.
  const stepX = width / Math.max(values.length - 1, 1)
  const pts = values.map((v, i) => {
    const clamped = Math.max(min, Math.min(upper, v))
    const x = i * stepX
    const y = height - ((clamped - min) / range) * height
    return [x, y] as const
  })

  const linePath = pts.map(([x, y], i) => (i === 0 ? `M${x},${y}` : `L${x},${y}`)).join(' ')
  const areaPath = `${linePath} L${pts[pts.length - 1][0]},${height} L${pts[0][0]},${height} Z`

  return (
    <svg
      className="sparkline"
      viewBox={`0 0 ${width} ${height}`}
      preserveAspectRatio="none"
      aria-hidden
    >
      <path d={areaPath} fill={color} opacity={0.18} />
      <path d={linePath} fill="none" stroke={color} strokeWidth={1.5}
            strokeLinejoin="round" strokeLinecap="round" vectorEffect="non-scaling-stroke" />
    </svg>
  )
}
