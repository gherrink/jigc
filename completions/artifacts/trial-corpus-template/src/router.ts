/**
 * A tiny method+path router over the store. Transport-agnostic on purpose —
 * `dispatch` takes strings and returns a plain object, so it is testable
 * without binding a socket.
 */

import type { Clock } from "./clock.ts";
import type { Config } from "./config.ts";
import { parseSample } from "./ingest.ts";
import { rollup } from "./rollup.ts";
import { MemoryStore } from "./store.ts";
import { formatSummary, summarize } from "./summary.ts";
import { ValidationError } from "./validate.ts";

export interface Reply {
  status: number;
  body: string;
}

export class Router {
  private readonly store: MemoryStore;
  private readonly config: Config;
  private readonly clock: Clock;

  constructor(store: MemoryStore, config: Config, clock: Clock) {
    this.store = store;
    this.config = config;
    this.clock = clock;
  }

  dispatch(method: string, path: string, body = ""): Reply {
    if (method === "POST" && path === "/samples") {
      return this.ingest(body);
    }
    if (method === "GET" && path === "/series") {
      return { status: 200, body: this.store.series().join("\n") };
    }
    if (method === "GET" && path.startsWith("/summary/")) {
      return this.summary(path.slice("/summary/".length));
    }
    return { status: 404, body: `no route for ${method} ${path}` };
  }

  private ingest(body: string): Reply {
    const lines = body.split("\n").filter((line) => line.trim() !== "");
    let accepted = 0;
    try {
      for (const line of lines) {
        this.store.put(parseSample(line, this.clock.now()));
        accepted += 1;
      }
    } catch (error) {
      if (error instanceof ValidationError) {
        return { status: 400, body: `${error.message} (accepted ${accepted})` };
      }
      throw error;
    }
    return { status: 202, body: `accepted ${accepted}` };
  }

  private summary(series: string): Reply {
    const windows = rollup(
      this.store,
      series,
      this.config.windowMs,
      this.clock.now(),
      this.config.retentionMs,
    );
    return { status: 200, body: formatSummary(summarize(windows, series)) };
  }
}

export function createRouter(
  store: MemoryStore,
  config: Config,
  clock: Clock,
): Router {
  return new Router(store, config, clock);
}
