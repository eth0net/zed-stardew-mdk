// Drive the language server directly and print its diagnostics, without Zed.
//
// Zed can't be scripted, so this is the only way to check that a schema
// association actually fires — which is how the example pack's own errors were
// found. It sends the same workspace configuration the extension builds.
//
//   node scripts/probe-lsp.mjs examples/ExampleContentPack/content.json
//   node scripts/probe-lsp.mjs 'path/with spaces/manifest.json' other.json
//
// Requires the extension to have been run once in Zed, so the server and the
// staged schemas exist in its work directory.

import { spawn } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { homedir } from 'node:os';

const work = `${homedir()}/Library/Application Support/Zed/extensions/work/stardew-mdk`;
const server = `${work}/node_modules/vscode-langservers-extracted/bin/vscode-json-language-server`;

const files = process.argv.slice(2).map((f) => resolve(f));
if (!files.length) {
  console.error('usage: node scripts/probe-lsp.mjs <file>...');
  process.exit(2);
}
for (const f of [server, ...files]) {
  if (!existsSync(f)) {
    console.error(`not found: ${f}`);
    process.exit(existsSync(server) ? 2 : 1);
  }
}

// Percent-encode as the extension does: the work directory contains a space.
const uri = (p) =>
  'file://' + p.replace(/[^A-Za-z0-9/:\-_.~]/g, (c) => '%' + c.charCodeAt(0).toString(16).toUpperCase());

const schema = (name, ...fileMatch) => ({ fileMatch, url: uri(`${work}/schemas/${name}`) });
const settings = {
  json: {
    validate: { enable: true },
    schemas: [
      schema('manifest.json', '**/manifest.json'),
      schema('i18n.json', '**/i18n/*.json', '**/i18n/**/*.json'),
      schema('content-patcher.json', '**/content.json'),
    ],
  },
};

const proc = spawn(process.execPath, [server, '--stdio']);
const seen = new Map();
let buf = Buffer.alloc(0);

proc.stdout.on('data', (chunk) => {
  buf = Buffer.concat([buf, chunk]);
  for (;;) {
    const split = buf.indexOf('\r\n\r\n');
    if (split < 0) return;
    const length = Number(/Content-Length: (\d+)/.exec(buf.subarray(0, split))[1]);
    if (buf.length < split + 4 + length) return;
    const msg = JSON.parse(buf.subarray(split + 4, split + 4 + length));
    buf = buf.subarray(split + 4 + length);
    if (msg.method === 'textDocument/publishDiagnostics') {
      seen.set(msg.params.uri, msg.params.diagnostics);
    }
  }
});

let id = 0;
const send = (msg) => {
  const body = JSON.stringify({ jsonrpc: '2.0', ...msg });
  proc.stdin.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
};
const settle = (ms) => new Promise((r) => setTimeout(r, ms));

send({ id: ++id, method: 'initialize', params: { processId: process.pid, rootUri: uri(process.cwd()), capabilities: {} } });
await settle(900);
send({ method: 'initialized', params: {} });
send({ method: 'workspace/didChangeConfiguration', params: { settings } });
await settle(400);

for (const file of files) {
  send({
    method: 'textDocument/didOpen',
    params: { textDocument: { uri: uri(file), languageId: 'jsonc', version: 1, text: readFileSync(file, 'utf8') } },
  });
}

// Schemas are fetched and resolved asynchronously; nothing signals completion.
await settle(4000);
proc.kill();

let total = 0;
for (const file of files) {
  const diagnostics = seen.get(uri(file));
  if (diagnostics === undefined) {
    console.log(`${file}\n    no schema matched`);
    continue;
  }
  total += diagnostics.length;
  console.log(`${file}\n    ${diagnostics.length} diagnostics`);
  for (const d of diagnostics) {
    console.log(`      line ${d.range.start.line + 1}: ${d.message.split('\n')[0]}`);
  }
}
process.exitCode = total === 0 ? 0 : 1;
