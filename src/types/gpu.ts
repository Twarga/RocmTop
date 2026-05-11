export interface GpuStats {
  temperature: number;
  gpu_clock: number;
  gpu_busy: number;
  vram_used: number;
  vram_total: number;
  power_mode: string;
  charger_status: boolean;
  runtime_pm: string;
}

export type PowerMode = 'high' | 'auto';

export type RuntimePmMode = 'on' | 'auto';
