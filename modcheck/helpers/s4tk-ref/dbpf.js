#!/usr/bin/env node
// Reference DBPF reader/writer for cross-validating ModCheck's Sims 4 inspector.
//
// S4TK (@s4tk/models, MIT) is an independent implementation of the DBPF
// container. This script exposes exactly two operations as JSON so the Python
// test suite can compare against it:
//
//   write <out.package> <keysJson>   build a package with the given resources
//   read  <in.package>               list the resource keys it contains
//
// It never executes package contents.

const fs = require('fs');
const { Package, RawResource } = require('@s4tk/models');

function keyString(key) {
  const type = key.type.toString(16).toUpperCase().padStart(8, '0');
  const group = key.group.toString(16).toUpperCase().padStart(8, '0');
  const instance = key.instance.toString(16).toUpperCase().padStart(16, '0');
  return `${type}:${group}:${instance}`;
}

function write(outPath, keysJson) {
  const specs = JSON.parse(keysJson);
  const pkg = new Package();
  specs.forEach((spec, i) => {
    pkg.add(
      { type: spec.type, group: spec.group, instance: BigInt(spec.instance) },
      RawResource.from(Buffer.from(spec.body ?? `resource-${i}`, 'utf8'))
    );
  });
  fs.writeFileSync(outPath, pkg.getBuffer());
  return { written: outPath, count: specs.length,
           keys: pkg.entries.map(e => keyString(e.key)) };
}

function read(inPath) {
  // loadRaw keeps resource payloads untouched. This comparison is about the
  // index -- the resource keys -- and decoding payloads would make the helper
  // fail on packages whose contents are placeholders.
  const pkg = Package.from(fs.readFileSync(inPath), { loadRaw: true });
  return {
    count: pkg.size,
    keys: pkg.entries.map(e => keyString(e.key)),
    entries: pkg.entries.map(e => ({
      key: keyString(e.key),
      type: e.key.type,
      group: e.key.group,
      instance: e.key.instance.toString(),
    })),
  };
}

const [, , command, ...rest] = process.argv;
try {
  let result;
  if (command === 'write') result = write(rest[0], rest[1]);
  else if (command === 'read') result = read(rest[0]);
  else { console.error('usage: dbpf.js write <out> <keysJson> | read <in>'); process.exit(2); }
  result.tool = '@s4tk/models';
  result.tool_version = require('@s4tk/models/package.json').version;
  console.log(JSON.stringify(result, null, 2));
} catch (err) {
  console.log(JSON.stringify({ error: String(err && err.message || err) }));
  process.exit(1);
}
