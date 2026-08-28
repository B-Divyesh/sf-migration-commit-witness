// @claim:build-artifacts
import { existsSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

const result = spawnSync('npm', ['run', 'build'], { stdio: 'inherit', shell: false });
if (result.status !== 0) process.exit(result.status ?? 1);
if (!existsSync('dist/bin/mcw') || !existsSync('dist/site/index.html') || !existsSync('dist/site/demo/index.html')) {
  throw new Error('build did not produce the documented CLI and site paths');
}

