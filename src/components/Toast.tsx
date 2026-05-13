import type { ToastState } from '../hooks/useToast'

interface ToastProps {
  toast: ToastState | null
  onDismiss: () => void
}

const ICONS: Record<ToastState['variant'], string> = {
  info: 'ℹ',
  success: '✓',
  error: '⚠',
}

export function Toast({ toast, onDismiss }: ToastProps) {
  if (!toast) return null

  return (
    <div
      key={toast.id}
      className={`toast toast-${toast.variant}`}
      role="status"
      aria-live="polite"
      onClick={onDismiss}
    >
      <span className="toast-icon" aria-hidden>{ICONS[toast.variant]}</span>
      <span className="toast-message">{toast.message}</span>
    </div>
  )
}
