import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const binary = resolve(process.platform === 'win32' ? 'target/release/mcw.exe' : 'target/release/mcw');
const parent = mkdtempSync(join(tmpdir(), 'mcw-recording-'));
const workspace = join(parent, 'workspace');
const transcript = execFileSync(binary, ['demo', '--output', workspace], {
  cwd: parent,
  encoding: 'utf8',
}).replaceAll(workspace, '/tmp/mcw-demo-<run>');

const escapeXml = (value) => value.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
const wrap = (line) => {
  if (line.length <= 72) return [line];
  const split = line.lastIndexOf(' ', 72);
  return [line.slice(0, split), line.slice(split + 1)];
};
const lines = ['$ mcw demo', ...transcript.trimEnd().split('\n').flatMap(wrap)];
const body = lines.map((line, index) => `<text x="36" y="${58 + index * 42}" fill="${index === 1 ? '#ff9b74' : '#dbe6d3'}">${escapeXml(line)}</text>`).join('');
const height = Math.max(360, 86 + lines.length * 42);
const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 760 ${height}" width="760" height="${height}" role="img" aria-labelledby="title desc"><title id="title">Recorded mcw demo terminal run</title><desc id="desc">A real bundled CLI sample detects a partial commit, restores the rollback value, and writes witness files in a temporary directory.</desc><rect width="760" height="${height}" fill="#090b09"/><rect x="18" y="18" width="724" height="${height - 36}" fill="#121512" stroke="#596157"/><circle cx="42" cy="39" r="6" fill="#ff9b74"/><circle cx="62" cy="39" r="6" fill="#f4ca72"/><circle cx="82" cy="39" r="6" fill="#b7d879"/><g font-family="ui-monospace, SFMono-Regular, Menlo, Consolas, monospace" font-size="16">${body}</g></svg>`;

writeFileSync('site/public/mcw-demo-recording.svg', svg);
const record = JSON.parse(readFileSync('site/public/demo-record.json', 'utf8'));
if (record.stages[2].schema !== '1 / 2' || record.stages[3].schema !== '0 / 2') throw new Error('Demo record no longer matches recording assumptions.');
