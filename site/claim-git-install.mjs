// @claim:git-install
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const repository = 'https://github.com/B-Divyesh/sf-migration-commit-witness';
const sandbox = mkdtempSync(join(tmpdir(), 'mcw-git-install-'));
const installRoot = join(sandbox, 'install-root');
const cargoHome = join(sandbox, 'cargo-home');
const binary = join(installRoot, 'bin', process.platform === 'win32' ? 'mcw.exe' : 'mcw');

try {
  const install = spawnSync('cargo', ['install', '--git', repository, '--bin', 'mcw'], {
    cwd: sandbox,
    env: { ...process.env, CARGO_HOME: cargoHome, CARGO_INSTALL_ROOT: installRoot },
    encoding: 'utf8',
    timeout: 600_000,
  });
  if (install.status !== 0) {
    process.stderr.write(install.stderr ?? 'cargo install failed\n');
    process.exit(install.status ?? 1);
  }
  if (!existsSync(binary)) throw new Error('documented Git installation did not create mcw');
  const version = execFileSync(binary, ['--version'], { encoding: 'utf8' }).trim();
  if (!/^mcw 0\.1\.1$/.test(version)) throw new Error(`unexpected installed version: ${version}`);
} finally {
  rmSync(sandbox, { recursive: true, force: true });
}
