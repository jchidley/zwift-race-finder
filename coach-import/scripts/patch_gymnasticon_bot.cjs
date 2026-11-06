"use strict";

// Match anchors derive from ptx2/gymnasticon (MIT), inspected compiled 1.4.0.
// Upstream attribution/licence: LICENSE-GYMNASTICON and README.md in this directory.
// Adapt the inspected installed 1.4.0 bot to the local safety helper.
// Generates a candidate file; never restarts services or edits the source file.
const fs = require("fs");
const [source, destination] = process.argv.slice(2);
if (!source || !destination || source === destination) {
  throw new Error("Usage: node patch_gymnasticon_bot.cjs SOURCE CANDIDATE");
}
let code = fs.readFileSync(source, "utf8");
if (code.includes('require("./bot-input.cjs")')) throw new Error("Already patched");

function replaceOnce(old, replacement) {
  if (code.split(old).length !== 2) throw new Error("Unexpected installed bot structure");
  code = code.replace(old, replacement);
}
function replaceMethod(signature, body) {
  const start = code.indexOf(signature);
  const end = code.indexOf("\n  /**", start);
  if (start < 0 || end < 0) throw new Error("Expected installed bot method missing");
  code = code.slice(0, start) + body + "\n" + code.slice(end);
}
replaceOnce('"use strict";', '"use strict";\nconst {BotInput} = require("./bot-input.cjs");');
replaceOnce("    this.power = power;\n    this.cadence = cadence;",
  "    this._input = new BotInput();\n    this.power = 0;\n    this.cadence = 0;\n    this._watchdog = null;");
replaceOnce("    await (0, _events.once)(this._udpServer, 'listening');",
  "    await (0, _events.once)(this._udpServer, 'listening');\n" +
  "    this._watchdog = setInterval(() => {\n" +
  "      if (this._input.expire()) this.onStatsUpdate();\n" +
  "    }, 100);");
replaceMethod("  onStatsUpdate() {", [
  "  onStatsUpdate() {",
  "    const sample = this._input.snapshot();",
  "    this.power = sample.power;",
  "    this.cadence = sample.cadence;",
  "    this.emit('stats', sample);",
  "  }"
].join("\n"));
replaceMethod("  onUdpMessage(msg, rinfo) {", [
  "  onUdpMessage(msg, rinfo) {",
  "    if (!this._input.accept(msg)) {",
  "      console.error('Rejected invalid bot sample; require integer power 0..32767 and cadence 0..254');",
  "      return;",
  "    }",
  "    this.onStatsUpdate();",
  "  }"
].join("\n"));
replaceOnce("  onUdpError(err) {",
  "  onUdpError(err) {\n" +
  "    this._input.clear();\n" +
  "    this.onStatsUpdate();\n" +
  "    clearInterval(this._watchdog);\n" +
  "    this._timer.cancel();");
fs.writeFileSync(destination, code, {flag: "wx", mode: 0o600});
console.log("Generated candidate bot: startup zero, full-sample validation, monotonic expiry.");
