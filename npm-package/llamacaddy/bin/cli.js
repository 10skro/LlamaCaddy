#!/usr/bin/env node
// LlamaCaddy launcher: resolves the bundled native binary and starts it detached.
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));

const candidates = [
  process.env.LLAMACADDY_BIN,
  path.join(here, "..", "bin", "LlamaCaddy.exe"),
].filter(Boolean);

const bin = candidates.find((p) => existsSync(p));
if (!bin) {
  console.error("LlamaCaddy binary not found. Reinstall the package or set LLAMACADDY_BIN.");
  process.exit(1);
}

const child = spawn(bin, process.argv.slice(2), { detached: true, stdio: "ignore" });
child.unref();
console.log(`LlamaCaddy started (pid ${child.pid})`);
