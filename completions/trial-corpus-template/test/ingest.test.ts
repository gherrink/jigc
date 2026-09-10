import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { IngestQueue, parseSample } from "../src/ingest.ts";
import { createService } from "../src/index.ts";
import { ValidationError } from "../src/validate.ts";

describe("parseSample", () => {
  it("parses the three-field wire form", () => {
    assert.deepEqual(parseSample("cpu.load 0.5 1000"), {
      series: "cpu.load",
      value: 0.5,
      at: 1000,
    });
  });

  it("falls back to the supplied timestamp when `at` is omitted", () => {
    assert.deepEqual(parseSample("cpu.load 0.5", 7000), {
      series: "cpu.load",
      value: 0.5,
      at: 7000,
    });
  });

  it("rejects a line with too many fields", () => {
    assert.throws(() => parseSample("cpu.load 1 2 3"), ValidationError);
  });

  it("rejects an omitted timestamp with no fallback", () => {
    assert.throws(() => parseSample("cpu.load 1"), /at:/);
  });
});

describe("IngestQueue", () => {
  it("drains what was pushed, in order", () => {
    const queue = new IngestQueue(4);
    queue.push({ series: "a", value: 1, at: 1 });
    queue.push({ series: "a", value: 2, at: 2 });
    assert.equal(queue.size(), 2);
    assert.deepEqual(
      queue.drain().map((s) => s.value),
      [1, 2],
    );
    assert.equal(queue.size(), 0);
  });

  it("drops the oldest sample on overflow", () => {
    const queue = new IngestQueue(2);
    for (const value of [1, 2, 3]) {
      queue.push({ series: "a", value, at: value });
    }
    assert.deepEqual(
      queue.drain().map((s) => s.value),
      [2, 3],
    );
    assert.equal(queue.droppedCount(), 1);
  });

  it("refuses a non-positive capacity", () => {
    assert.throws(() => new IngestQueue(0), RangeError);
  });
});

describe("the ingest path", () => {
  it("buffers a posted sample and lands it on the next tick", () => {
    const service = createService(
      { windowMs: 1_000, retentionMs: 60_000, maxSamples: 4 },
      { now: () => 1_000 },
    );

    const reply = service.router.dispatch("POST", "/samples", "cpu.load 0.5 1000");
    assert.equal(reply.status, 202);
    assert.deepEqual(
      service.store.series(),
      [],
      "a posted sample is buffered, not stored — the store is only current after a tick",
    );

    service.tick();
    assert.deepEqual(service.store.series(), ["cpu.load"]);
  });
});
