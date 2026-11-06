"use strict";
// Radio-free integration test against a generated/installed bot candidate.
const assert = require("assert");
const fs = require("fs");
const path = require("path");
const vm = require("vm");
const events = require("events");
const {BotInput} = require("./gymnasticon_bot_input.cjs");
const candidate = process.argv[2];
if (!candidate) throw new Error("Provide candidate bot path");
let now = 0, poll = null, cleared = false;
class Input extends BotInput {
  constructor() { super(() => now); }
}
class Timer extends events.EventEmitter {
  reset() {}
  cancel() { this.cancelled = true; }
}
class Socket extends events.EventEmitter {
  bind() { process.nextTick(() => this.emit("listening")); }
}
const socket = new Socket();
const context = {
  exports: {},
  console: {error() {}},
  setInterval(callback, ms) { assert.strictEqual(ms, 100); poll = callback; return 1; },
  clearInterval() { cleared = true; },
  require(name) {
    if (name === "./bot-input.cjs") return {BotInput: Input};
    if (name === "dgram") return {createSocket: () => socket};
    if (name === "events") return events;
    if (name === "../util/timer") return {Timer};
    throw new Error("Unexpected dependency: " + name);
  }
};
vm.runInNewContext(fs.readFileSync(candidate, "utf8"), context, {filename: path.basename(candidate)});
(async () => {
  const bot = new context.exports.BotBikeClient(500, 100, "127.0.0.1", 3000);
  let last;
  bot.on("stats", sample => { last = JSON.parse(JSON.stringify(sample)); });
  let disconnects = 0;
  bot.on("disconnect", () => disconnects++);
  await bot.connect();
  bot.onStatsUpdate();
  assert.deepStrictEqual(last, {power: 0, cadence: 0});
  socket.emit("message", Buffer.from('{"power":180,"cadence":85}'));
  assert.deepStrictEqual(last, {power: 180, cadence: 85});
  now = 2499;
  poll();
  assert.deepStrictEqual(last, {power: 180, cadence: 85});
  socket.emit("message", Buffer.from("{bad"));
  socket.emit("message", Buffer.from('{"power":999}'));
  now = 2500;
  poll();
  assert.deepStrictEqual(last, {power: 0, cadence: 0});
  now = 3000;
  socket.emit("message", Buffer.from('{"power":150,"cadence":80}'));
  assert.deepStrictEqual(last, {power: 150, cadence: 80});
  now = 5400;
  socket.emit("message", Buffer.from('{"power":150,"cadence":255}'));
  now = 5500;
  poll();
  assert.deepStrictEqual(last, {power: 0, cadence: 0});
  socket.emit("message", Buffer.from('{"power":100,"cadence":70}'));
  socket.emit("error", new Error("test"));
  assert.deepStrictEqual(last, {power: 0, cadence: 0});
  assert.strictEqual(disconnects, 1);
  assert.strictEqual(cleared, true);
  assert.strictEqual(bot._timer.cancelled, true);
  console.log("PASS: installed adapter startup, immediate sample, watchdog emission, invalid traffic, recovery, socket-error zero/cleanup.");
})().catch(error => { console.error(error); process.exitCode = 1; });
