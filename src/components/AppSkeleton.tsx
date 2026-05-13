import { Skeleton } from './Skeleton'

/**
 * First-paint placeholder that replicates the exact layout of the populated
 * app so there's no content shift when `get_all_stats` returns. Rendered only
 * while `stats === null`.
 */
export function AppSkeleton() {
  return (
    <div className="app">
      <div className="header">
        <h1>RocmTop</h1>
        <Skeleton width={64} height={24} radius={4} />
      </div>

      <div className="metrics-grid">
        {[0, 80, 160, 240].map((delay, i) => (
          <div key={i} className="metric-card">
            <Skeleton width={72} height={10} delay={delay} />
            <Skeleton width={110} height={28} radius={6} delay={delay + 40} />
            <Skeleton width="100%" height={4} radius={2} delay={delay + 80} />
            <Skeleton width="100%" height={28} radius={4} delay={delay + 120} />
          </div>
        ))}
      </div>

      <div className="status-section">
        <div className="status-row">
          <Skeleton width={88} height={12} delay={200} />
          <Skeleton width={120} height={24} radius={4} delay={240} />
        </div>
        <div className="status-row">
          <Skeleton width={88} height={12} delay={260} />
          <Skeleton width={120} height={24} radius={4} delay={300} />
        </div>
        <div className="status-row">
          <Skeleton width={60} height={12} delay={320} />
          <Skeleton width={90} height={14} delay={360} />
        </div>
      </div>

      <div className="ai-section">
        <Skeleton width={80} height={11} delay={380} />
        <div className="ai-buttons">
          <Skeleton width="100%" height={36} radius={6} delay={400} />
          <Skeleton width="100%" height={36} radius={6} delay={440} />
        </div>
      </div>

      <div className="footer">
        <Skeleton width={120} height={10} delay={480} />
        <Skeleton width={36} height={10} delay={520} />
      </div>
    </div>
  )
}
