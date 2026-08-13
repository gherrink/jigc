/**
 * In-memory sample storage, one sorted bucket per series.
 *
 * Deliberately not durable: this service is a rollup cache in front of whatever
 * long-term store the caller already has.
 */

import type { Sample } from "./validate.ts";

export class MemoryStore {
  private readonly buckets = new Map<string, Sample[]>();
  private readonly maxPerSeries: number;

  constructor(maxPerSeries: number) {
    if (!Number.isInteger(maxPerSeries) || maxPerSeries <= 0) {
      throw new RangeError("maxPerSeries must be a positive integer");
    }
    this.maxPerSeries = maxPerSeries;
  }

  /** Insert a sample, keeping the series bucket ordered by timestamp. */
  put(sample: Sample): void {
    let bucket = this.buckets.get(sample.series);
    if (bucket === undefined) {
      bucket = [];
      this.buckets.set(sample.series, bucket);
    }
    const at = insertionPoint(bucket, sample.at);
    bucket.splice(at, 0, sample);
    if (bucket.length > this.maxPerSeries) {
      bucket.splice(0, bucket.length - this.maxPerSeries);
    }
  }

  /** Samples for `series` with `from <= at < to`, in timestamp order. */
  range(series: string, from: number, to: number): Sample[] {
    const bucket = this.buckets.get(series);
    if (bucket === undefined) return [];
    return bucket.filter((s) => s.at >= from && s.at < to);
  }

  /** Every series name currently holding at least one sample. */
  series(): string[] {
    return [...this.buckets.keys()].sort();
  }

  /** Drop every sample older than `before`; returns how many were removed. */
  prune(before: number): number {
    let removed = 0;
    for (const [name, bucket] of this.buckets) {
      const keep = bucket.filter((s) => s.at >= before);
      removed += bucket.length - keep.length;
      if (keep.length === 0) {
        this.buckets.delete(name);
      } else {
        this.buckets.set(name, keep);
      }
    }
    return removed;
  }

  size(): number {
    let total = 0;
    for (const bucket of this.buckets.values()) total += bucket.length;
    return total;
  }
}

/** Binary search for the index at which `at` keeps `bucket` sorted. */
function insertionPoint(bucket: Sample[], at: number): number {
  let low = 0;
  let high = bucket.length;
  while (low < high) {
    const mid = (low + high) >>> 1;
    if (bucket[mid].at <= at) low = mid + 1;
    else high = mid;
  }
  return low;
}
