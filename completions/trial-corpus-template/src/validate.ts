/**
 * The one place a sample is admitted or rejected. Everything downstream may
 * assume a Sample is well-formed.
 */

export interface Sample {
  /** Series name — non-empty, no whitespace. */
  series: string;
  /** The measured value; finite. */
  value: number;
  /** Epoch milliseconds. */
  at: number;
}

export class ValidationError extends Error {
  readonly field: string;

  constructor(field: string, message: string) {
    super(`${field}: ${message}`);
    this.name = "ValidationError";
    this.field = field;
  }
}

export function isFiniteNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

const SERIES_PATTERN = /^[a-z][a-z0-9_.-]*$/;

export function isValidSeries(name: unknown): name is string {
  return typeof name === "string" && SERIES_PATTERN.test(name);
}

/**
 * Coerce an unknown payload into a Sample, or throw. Never returns a partially
 * valid object — callers get all-or-nothing.
 */
export function validateSample(input: unknown): Sample {
  if (input === null || typeof input !== "object") {
    throw new ValidationError("sample", "expected an object");
  }
  const candidate = input as Record<string, unknown>;

  if (!isValidSeries(candidate.series)) {
    throw new ValidationError(
      "series",
      "expected a lowercase name matching [a-z][a-z0-9_.-]*",
    );
  }
  if (!isFiniteNumber(candidate.value)) {
    throw new ValidationError("value", "expected a finite number");
  }
  if (!isFiniteNumber(candidate.at) || !Number.isInteger(candidate.at)) {
    throw new ValidationError("at", "expected epoch milliseconds as an integer");
  }
  if (candidate.at < 0) {
    throw new ValidationError("at", "expected a non-negative timestamp");
  }

  return { series: candidate.series, value: candidate.value, at: candidate.at };
}
