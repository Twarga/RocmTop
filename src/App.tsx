import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { GpuStats } from './types/gpu'
import { useToast } from './hooks/useToast'
import { useGpuHistory } from './hooks/useGpuHistory'
import { Toast } from './components/Toast'
import { AnimatedNumber } from './components/AnimatedNumber'
import { Sparkline } from './components/Sparkline'
import { AppSkeleton } from './components/AppSkeleton'

function App() {
  const [stats, setStats] = useState<GpuStats | null>(null)
  const [lastUpdate, setLastUpdate] = useState<Date>(new Date())
  const { toast, show, dismiss } = useToast()
  const history = useGpuHistory(stats)

  useEffect(() => {
    let cancelled = false

    const tick = async () => {
      try {
        const data = await invoke<GpuStats>('get_all_stats')
        if (!cancelled) {
          setStats(data)
          setLastUpdate(new Date())
        }
      } catch (e) {
        if (!cancelled) show(`Failed to read GPU stats: ${formatError(e)}`, 'error')
      }
    }

    tick()
    const poll = setInterval(tick, 2000)
    return () => {
      cancelled = true
      clearInterval(poll)
    }
  }, [show])

  const getTempColor = (temp: number) => {
    if (temp < 80) return 'temp-normal'
    if (temp <= 88) return 'temp-warm'
    return 'temp-hot'
  }

  const tempSparklineColor = (temp: number) => {
    if (temp < 80) return 'var(--accent-green)'
    if (temp <= 88) return 'var(--accent-yellow)'
    return 'var(--accent-red)'
  }

  const getProgressColor = (temp: number) => {
    if (temp < 80) return 'progress-green'
    if (temp <= 88) return 'progress-yellow'
    return 'progress-red'
  }

  const setPowerMode = async (mode: string) => {
    try {
      await invoke('set_power_mode', { mode })
      show(`Power mode → ${mode.toUpperCase()}`, 'success')
    } catch (e) {
      show(`Power mode failed: ${formatError(e)}`, 'error')
    }
  }

  const setRuntimePm = async (mode: string) => {
    try {
      await invoke('set_runtime_pm', { mode })
      show(`Runtime PM → ${mode.toUpperCase()}`, 'success')
    } catch (e) {
      show(`Runtime PM failed: ${formatError(e)}`, 'error')
    }
  }

  const startAiSession = async () => {
    try {
      await invoke('start_ai_session')
      show('AI session started (HIGH + PM ON)', 'success')
    } catch (e) {
      show(`Start failed: ${formatError(e)}`, 'error')
    }
  }

  const endAiSession = async () => {
    try {
      await invoke('end_ai_session')
      show('AI session ended (AUTO)', 'success')
    } catch (e) {
      show(`End failed: ${formatError(e)}`, 'error')
    }
  }

  const vramUsedMb = stats ? Math.round(stats.vram_used / 1048576) : 0
  const vramTotalMb = stats ? Math.round(stats.vram_total / 1048576) : 0
  const vramPercent = vramTotalMb > 0 ? (vramUsedMb / vramTotalMb) * 100 : 0

  const secondsAgo = Math.floor((new Date().getTime() - lastUpdate.getTime()) / 1000)

  if (!stats) {
    return (
      <>
        <AppSkeleton />
        <Toast toast={toast} onDismiss={dismiss} />
      </>
    )
  }

  return (
    <div className="app">
      <div className="header">
        <h1>RocmTop</h1>
        <button className="refresh-btn" onClick={() => setLastUpdate(new Date())}>
          Refresh
        </button>
      </div>

      <div className="metrics-grid">
        <div className="metric-card">
          <h3>Temperature</h3>
          <div className={`metric-value ${getTempColor(stats.temperature)}`}>
            <AnimatedNumber value={stats.temperature} /><span className="metric-unit">°C</span>
          </div>
          <div className="progress-bar">
            <div 
              className={`progress-fill ${getProgressColor(stats.temperature)}`}
              style={{ width: `${Math.min((stats.temperature / 100) * 100, 100)}%` }}
            />
          </div>
          <Sparkline values={history.temperature} max={100} color={tempSparklineColor(stats.temperature)} />
        </div>

        <div className="metric-card">
          <h3>GPU Clock</h3>
          <div className="metric-value">
            <AnimatedNumber value={stats.gpu_clock} /><span className="metric-unit">MHz</span>
          </div>
          <div className="progress-bar">
            <div 
              className="progress-fill progress-blue"
              style={{ width: `${stats.max_clock > 0 ? Math.min((stats.gpu_clock / stats.max_clock) * 100, 100) : 0}%` }}
            />
          </div>
          <Sparkline values={history.gpuClock} max={stats.max_clock} color="var(--accent-blue)" />
        </div>

        <div className="metric-card">
          <h3>GPU Load</h3>
          <div className="metric-value">
            <AnimatedNumber value={stats.gpu_busy} /><span className="metric-unit">%</span>
          </div>
          <div className="progress-bar">
            <div 
              className="progress-fill progress-blue"
              style={{ width: `${stats.gpu_busy}%` }}
            />
          </div>
          <Sparkline values={history.gpuBusy} max={100} color="var(--accent-blue)" />
        </div>

        <div className="metric-card">
          <h3>VRAM</h3>
          <div className="metric-value">
            <AnimatedNumber value={vramUsedMb} /><span className="metric-unit"> / {vramTotalMb} MB</span>
          </div>
          <div className="progress-bar">
            <div 
              className="progress-fill progress-blue"
              style={{ width: `${vramPercent}%` }}
            />
          </div>
          <Sparkline values={history.vramUsedMb} max={vramTotalMb} color="var(--accent-blue)" />
        </div>
      </div>

      <div className="status-section">
        <div className="status-row">
          <span className="status-label">Power Mode</span>
          <div className="toggle-group">
            <button 
              className={`toggle-btn ${stats.power_mode === 'high' ? 'active' : ''}`}
              onClick={() => setPowerMode('high')}
            >
              HIGH
            </button>
            <button 
              className={`toggle-btn ${stats.power_mode === 'auto' ? 'active' : ''}`}
              onClick={() => setPowerMode('auto')}
            >
              AUTO
            </button>
          </div>
        </div>

        <div className="status-row">
          <span className="status-label">Runtime PM</span>
          <div className="toggle-group">
            <button 
              className={`toggle-btn ${stats.runtime_pm === 'on' ? 'active' : ''}`}
              onClick={() => setRuntimePm('on')}
            >
              ON
            </button>
            <button 
              className={`toggle-btn ${stats.runtime_pm === 'auto' ? 'active' : ''}`}
              onClick={() => setRuntimePm('auto')}
            >
              AUTO
            </button>
          </div>
        </div>

        <div className="status-row">
          <span className="status-label">Charger</span>
          <span className={`charger-status ${stats.charger_status ? 'plugged' : 'battery'}`}>
            {stats.charger_status ? 'Plugged in ✓' : 'On battery'}
          </span>
        </div>
      </div>

      <div className="ai-section">
        <h3>AI Session</h3>
        <div className="ai-buttons">
          <button className="ai-btn start" onClick={startAiSession}>
            Start AI Session
          </button>
          <button className="ai-btn end" onClick={endAiSession}>
            End AI Session
          </button>
        </div>
      </div>

      <div className="footer">
        <span>Last updated: {secondsAgo}s ago</span>
        <span>v1.0.0</span>
      </div>

      <Toast toast={toast} onDismiss={dismiss} />
    </div>
  )
}

/**
 * Convert unknown Tauri invoke error to short human-readable text.
 * The Rust side returns a String via Result<(), String>; Tauri passes it
 * through as the rejected value. Keep the toast compact.
 */
function formatError(e: unknown): string {
  if (typeof e === 'string') return e
  if (e instanceof Error) return e.message
  try {
    return JSON.stringify(e)
  } catch {
    return String(e)
  }
}

export default App
