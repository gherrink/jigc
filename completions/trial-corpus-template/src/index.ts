/**
 * Wiring. Everything above is injectable; this is the one place that decides
 * which real implementations get used.
 */

import type { Clock } from "./clock.ts";
import { SystemClock } from "./clock.ts";
import type { Config } from "./config.ts";
import { loadConfig } from "./config.ts";
import { IngestQueue } from "./ingest.ts";
import { MemoryStore } from "./store.ts";
import type { Router } from "./router.ts";
import { createRouter } from "./router.ts";

export interface Service {
  config: Config;
  store: MemoryStore;
  queue: IngestQueue;
  router: Router;
  /** Move queued samples into the store and expire anything past retention. */
  tick(): void;
}

export function createService(
  config: Config = loadConfig(),
  clock: Clock = new SystemClock(),
): Service {
  const store = new MemoryStore(config.maxSamples);
  const queue = new IngestQueue(config.maxSamples);
  const router = createRouter(store, queue, config, clock);

  return {
    config,
    store,
    queue,
    router,
    tick(): void {
      for (const sample of queue.drain()) store.put(sample);
      store.prune(clock.now() - config.retentionMs);
    },
  };
}

export function main(): void {
  const service = createService();
  // Drain whatever the queue holds and expire anything past retention before
  // reporting: the numbers below are about the store, and the store is only
  // current once the queue has been moved into it.
  service.tick();
  const series = service.store.series();
  process.stdout.write(
    `rollup service ready — window ${service.config.windowMs}ms, ` +
      `${series.length} series, ${service.queue.droppedCount()} dropped\n`,
  );
}

if (process.argv[1] && process.argv[1].endsWith("index.ts")) {
  main();
}
