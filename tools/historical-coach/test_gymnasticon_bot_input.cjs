"use strict";
const assert = require("assert");
const {BotInput, FRESHNESS_MS} = require("./gymnasticon_bot_input.cjs");
let now = 0;
const state = new BotInput(() => now);
const send = sample => state.accept(Buffer.from(JSON.stringify(sample)));
const zero = {power: 0, cadence: 0};
assert.deepStrictEqual(state.snapshot(), zero);
assert(send({power: 180, cadence: 85}));
now = FRESHNESS_MS - 1;
assert.deepStrictEqual(state.snapshot(), {power: 180, cadence: 85});
assert.strictEqual(state.accept(Buffer.from("{bad")), false);
for (const invalid of [null, [], 42, "x", {}, {power: 1}, {cadence: 1},
  {power: -1, cadence: 85}, {power: 180, cadence: -1},
  {power: 1.5, cadence: 85}, {power: 180, cadence: 85.5},
  {power: 32768, cadence: 85}, {power: 65535, cadence: 85}, {power: 180, cadence: 255},
  {power: "180", cadence: 85}]) {
  assert.strictEqual(send(invalid), false);
}
now = FRESHNESS_MS;
assert.deepStrictEqual(state.snapshot(), zero); // Invalid input did not refresh.
assert(send({power: 32767, cadence: 254}));
assert.deepStrictEqual(state.snapshot(), {power: 32767, cadence: 254});
now += 2000;
assert(send({power: 200, cadence: 90}));
now += 2000;
assert.deepStrictEqual(state.snapshot(), {power: 200, cadence: 90});
assert(send(zero));
assert.deepStrictEqual(state.snapshot(), zero);
now += FRESHNESS_MS;
assert.strictEqual(state.expire(), true);
assert.strictEqual(state.expire(), false);
state.clear();
assert.deepStrictEqual(state.snapshot(), zero);
console.log("PASS: startup, valid/boundary samples, deadline, invalid/malformed/partial input, refresh, zero, recovery.");
