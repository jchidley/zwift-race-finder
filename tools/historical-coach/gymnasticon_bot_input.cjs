"use strict";

// Original local safety helper, not vendored Gymnasticon source.
// Expire at 2.5 s, leaving 0.5 s for watchdog polling and ANT scheduling
// within the 3 s bench acceptance deadline. Not a hard-real-time guarantee.
const FRESHNESS_MS = 2500;

class BotInput {
  constructor(now = () => Number(process.hrtime.bigint() / 1000000n)) {
    this.now = now;
    this.clear();
  }

  clear() {
    this.power = 0;
    this.cadence = 0;
    this.receivedAt = null;
  }

  accept(message) {
    let sample;
    try {
      sample = JSON.parse(message.toString());
    } catch (_) {
      return false;
    }
    // Require complete samples: partial updates must not refresh old fields.
    if (!sample || typeof sample !== "object" || Array.isArray(sample) ||
        !Number.isInteger(sample.power) || sample.power < 0 || sample.power > 32767 ||
        !Number.isInteger(sample.cadence) || sample.cadence < 0 || sample.cadence > 254) {
      return false;
    }
    this.power = sample.power;
    this.cadence = sample.cadence;
    this.receivedAt = this.now();
    return true;
  }

  expire() {
    if (this.receivedAt !== null && this.now() - this.receivedAt >= FRESHNESS_MS) {
      this.clear();
      return true;
    }
    return false;
  }

  snapshot() {
    this.expire();
    return {power: this.power, cadence: this.cadence};
  }
}

module.exports = {BotInput, FRESHNESS_MS};
