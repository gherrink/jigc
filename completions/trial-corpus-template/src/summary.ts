/**
 * Collapse a window series into a single human- and wire-readable summary.
 */

import type { Window } from "./rollup.ts";

export interface Summary {
  series: string;
  /** Number of samples across every window. */
  count: number;
  mean: number;
  min: number;
  max: number;
  /** Bounds of the covered span, or null when there is nothing to cover. */
  from: number | null;
  to: number | null;
}

export function summarize(windows: Window[], series: string): Summary {
  if (windows.length === 0) {
    return { series, count: 0, mean: 0, min: 0, max: 0, from: null, to: null };
  }

  let count = 0;
  let sum = 0;
  let min = Infinity;
  let max = -Infinity;

  for (const window of windows) {
    count += window.count;
    sum += window.sum;
    min = Math.min(min, window.min);
    max = Math.max(max, window.max);
  }

  return {
    series,
    count,
    mean: count === 0 ? 0 : sum / count,
    min,
    max,
    from: windows[0].from,
    to: windows[windows.length - 1].to,
  };
}

export function formatSummary(summary: Summary): string {
  if (summary.count === 0) {
    return `${summary.series}: no samples`;
  }
  const mean = summary.mean.toFixed(2);
  return `${summary.series}: n=${summary.count} mean=${mean} min=${summary.min} max=${summary.max}`;
}
