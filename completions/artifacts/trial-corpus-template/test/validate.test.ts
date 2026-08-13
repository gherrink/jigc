import assert from "node:assert/strict";
import { describe, it } from "node:test";

import {
  ValidationError,
  isFiniteNumber,
  isValidSeries,
  validateSample,
} from "../src/validate.ts";

describe("validateSample", () => {
  it("accepts a well-formed sample", () => {
    const sample = validateSample({ series: "cpu.load", value: 0.42, at: 1000 });
    assert.deepEqual(sample, { series: "cpu.load", value: 0.42, at: 1000 });
  });

  it("rejects a non-object", () => {
    assert.throws(() => validateSample("cpu.load 1 2"), ValidationError);
  });

  it("names the offending field", () => {
    try {
      validateSample({ series: "cpu.load", value: Number.NaN, at: 1000 });
      assert.fail("expected a ValidationError");
    } catch (error) {
      assert.ok(error instanceof ValidationError);
      assert.equal(error.field, "value");
    }
  });

  it("rejects a fractional timestamp", () => {
    assert.throws(
      () => validateSample({ series: "cpu.load", value: 1, at: 10.5 }),
      /at:/,
    );
  });

  it("rejects an upper-case series name", () => {
    assert.throws(
      () => validateSample({ series: "CPU", value: 1, at: 10 }),
      /series:/,
    );
  });
});

describe("isValidSeries", () => {
  it("accepts dotted lowercase names", () => {
    assert.equal(isValidSeries("http.requests_total"), true);
  });

  it("rejects a leading digit", () => {
    assert.equal(isValidSeries("5xx"), false);
  });
});

describe("isFiniteNumber", () => {
  it("rejects Infinity", () => {
    assert.equal(isFiniteNumber(Number.POSITIVE_INFINITY), false);
  });
});
