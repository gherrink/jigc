import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { FixedClock } from "../src/clock.ts";
import { DEFAULT_CONFIG } from "../src/config.ts";
import { rollup, windowStart, windowsFor } from "../src/rollup.ts";
import { MemoryStore } from "../src/store.ts";
import { formatSummary, summarize } from "../src/summary.ts";

describe("windowStart", () => {
  it("aligns down to a multiple of the width", () => {
    assert.equal(windowStart(1_234, 1_000), 1_000);
    assert.equal(windowStart(1_000, 1_000), 1_000);
  });
});

describe("windowsFor", () => {
  it("buckets samples and carries min/max", () => {
    const windows = windowsFor(
      [
        { series: "a", value: 1, at: 0 },
        { series: "a", value: 3, at: 500 },
        { series: "a", value: 9, at: 1_500 },
      ],
      1_000,
    );
    assert.equal(windows.length, 2);
    assert.deepEqual(
      windows.map((w) => [w.from, w.count, w.min, w.max]),
      [
        [0, 2, 1, 3],
        [1_000, 1, 9, 9],
      ],
    );
  });

  it("omits empty windows rather than emitting zeroes", () => {
    const windows = windowsFor(
      [
        { series: "a", value: 1, at: 0 },
        { series: "a", value: 1, at: 5_000 },
      ],
      1_000,
    );
    assert.deepEqual(
      windows.map((w) => w.from),
      [0, 5_000],
    );
  });

  it("refuses a zero-width window", () => {
    assert.throws(() => windowsFor([], 0), RangeError);
  });
});

describe("rollup", () => {
  it("covers the trailing span only", () => {
    const store = new MemoryStore(100);
    store.put({ series: "a", value: 1, at: 1_000 });
    store.put({ series: "a", value: 5, at: 9_000 });

    const clock = new FixedClock(10_000);
    const windows = rollup(store, "a", 1_000, clock.now(), 2_000);

    assert.deepEqual(
      windows.map((w) => w.from),
      [9_000],
    );
  });
});

describe("summarize", () => {
  it("reports the empty case without inventing a measurement", () => {
    const summary = summarize([], "a");
    assert.equal(summary.count, 0);
    assert.equal(summary.from, null);
    assert.equal(formatSummary(summary), "a: no samples");
  });

  it("means across windows, not across window means", () => {
    const windows = windowsFor(
      [
        { series: "a", value: 1, at: 0 },
        { series: "a", value: 2, at: 1 },
        { series: "a", value: 9, at: 1_000 },
      ],
      1_000,
    );
    const summary = summarize(windows, "a");
    assert.equal(summary.count, 3);
    assert.equal(summary.mean, 4);
  });
});

describe("DEFAULT_CONFIG", () => {
  it("keeps a day of retention", () => {
    assert.equal(DEFAULT_CONFIG.retentionMs, 86_400_000);
  });
});
