/**
 * The write edge: text lines in, validated Samples out, buffered until drained.
 */

import type { Sample } from "./validate.ts";
import { ValidationError, validateSample } from "./validate.ts";

/**
 * Parse one wire line into a Sample.
 *
 * Wire format is three whitespace-separated fields: `<series> <value> <at>`.
 * A trailing timestamp may be omitted, in which case `fallbackAt` is used.
 */
export function parseSample(line: string, fallbackAt?: number): Sample {
  const fields = line.trim().split(/\s+/);
  if (fields.length < 2 || fields.length > 3) {
    throw new ValidationError(
      "line",
      `expected "<series> <value> [<at>]", got ${fields.length} field(s)`,
    );
  }
  const [series, rawValue, rawAt] = fields;

  const at = rawAt === undefined ? fallbackAt : Number(rawAt);
  if (at === undefined) {
    throw new ValidationError("at", "omitted, and no fallback timestamp given");
  }

  return validateSample({ series, value: Number(rawValue), at });
}

/**
 * A bounded FIFO buffer between the wire and the store.
 *
 * Overflow drops the *oldest* sample, not the newest: under sustained pressure
 * a recent picture beats a stale one.
 */
export class IngestQueue {
  private buffer: Sample[] = [];
  private dropped = 0;
  private readonly capacity: number;

  constructor(capacity: number) {
    if (!Number.isInteger(capacity) || capacity <= 0) {
      throw new RangeError("capacity must be a positive integer");
    }
    this.capacity = capacity;
  }

  push(sample: Sample): void {
    this.buffer.push(sample);
    while (this.buffer.length > this.capacity) {
      this.buffer.shift();
      this.dropped += 1;
    }
  }

  /** Take everything buffered so far, leaving the queue empty. */
  drain(): Sample[] {
    const taken = this.buffer;
    this.buffer = [];
    return taken;
  }

  size(): number {
    return this.buffer.length;
  }

  /** How many samples overflow has discarded since construction. */
  droppedCount(): number {
    return this.dropped;
  }
}
