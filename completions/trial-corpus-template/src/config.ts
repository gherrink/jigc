/**
 * Runtime configuration, read once at startup from the environment.
 */

export interface Config {
  /** Width of a rollup window, in milliseconds. */
  windowMs: number;
  /** Hard cap on samples held per series before the oldest are dropped. */
  maxSamples: number;
  /** How long a sample stays queryable, in milliseconds. */
  retentionMs: number;
}

export class ConfigError extends Error {
  readonly key: string;

  constructor(key: string, message: string) {
    super(`${key}: ${message}`);
    this.name = "ConfigError";
    this.key = key;
  }
}

export const DEFAULT_CONFIG: Config = {
  windowMs: 60_000,
  maxSamples: 10_000,
  retentionMs: 24 * 60 * 60 * 1000,
};

function readPositiveInt(
  env: Record<string, string | undefined>,
  key: string,
  fallback: number,
): number {
  const raw = env[key];
  if (raw === undefined || raw === "") return fallback;
  const parsed = Number(raw);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new ConfigError(key, `expected a positive integer, got "${raw}"`);
  }
  return parsed;
}

/**
 * Build a Config from the environment, falling back to DEFAULT_CONFIG per key.
 * Throws ConfigError on a present-but-unusable value rather than silently
 * falling back — a typo in a deploy env should be loud.
 */
export function loadConfig(
  env: Record<string, string | undefined> = process.env,
): Config {
  return {
    windowMs: readPositiveInt(env, "ROLLUP_WINDOW_MS", DEFAULT_CONFIG.windowMs),
    maxSamples: readPositiveInt(env, "ROLLUP_MAX_SAMPLES", DEFAULT_CONFIG.maxSamples),
    retentionMs: readPositiveInt(env, "ROLLUP_RETENTION_MS", DEFAULT_CONFIG.retentionMs),
  };
}
