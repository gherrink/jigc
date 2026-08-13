/**
 * Fixed-width window aggregation over stored samples.
 */

import type { MemoryStore } from "./store.ts";
import type { Sample } from "./validate.ts";

export interface Window {
  /** Inclusive lower bound, epoch ms, aligned to a multiple of windowMs. */
  from: number;
  /** Exclusive upper bound, epoch ms. */
  to: number;
  count: number;
  sum: number;
  min: number;
  max: number;
}

/** The aligned window start containing `at`. */
export function windowStart(at: number, windowMs: number): number {
  return Math.floor(at / windowMs) * windowMs;
}

/**
 * Bucket samples into aligned windows. Empty windows are omitted rather than
 * emitted as zeroes — a gap is a gap, not a measurement of zero.
 */
export function windowsFor(samples: Sample[], windowMs: number): Window[] {
  if (!Number.isInteger(windowMs) || windowMs <= 0) {
    throw new RangeError("windowMs must be a positive integer");
  }
  const byStart = new Map<number, Window>();

  for (const sample of samples) {
    const from = windowStart(sample.at, windowMs);
    let window = byStart.get(from);
    if (window === undefined) {
      window = {
        from,
        to: from + windowMs,
        count: 0,
        sum: 0,
        min: sample.value,
        max: sample.value,
      };
      byStart.set(from, window);
    }
    window.count += 1;
    window.sum += sample.value;
    window.min = Math.min(window.min, sample.value);
    window.max = Math.max(window.max, sample.value);
  }

  return [...byStart.values()].sort((a, b) => a.from - b.from);
}

/**
 * Roll up one series over the trailing `spanMs` ending at `now`.
 */
export function rollup(
  store: MemoryStore,
  series: string,
  windowMs: number,
  now: number,
  spanMs: number,
): Window[] {
  const from = windowStart(now - spanMs, windowMs);
  const samples = store.range(series, from, now + 1);
  return windowsFor(samples, windowMs);
}
