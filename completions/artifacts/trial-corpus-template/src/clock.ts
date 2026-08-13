/**
 * Time is injected, never read from the global — every window boundary and
 * retention sweep is testable because of it.
 */

export interface Clock {
  now(): number;
}

export class SystemClock implements Clock {
  now(): number {
    return Date.now();
  }
}

/** A clock that only moves when you move it. Used by the tests. */
export class FixedClock implements Clock {
  private current: number;

  constructor(current: number) {
    this.current = current;
  }

  now(): number {
    return this.current;
  }

  advance(ms: number): void {
    if (ms < 0) throw new RangeError("cannot advance a clock backwards");
    this.current += ms;
  }

  set(at: number): void {
    this.current = at;
  }
}
