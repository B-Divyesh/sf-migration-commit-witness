// @claim:toolchain-compatibility
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const minimumNode = 22;
const minimumRust = [1, 85, 0];
const packageJson = JSON.parse(readFileSync('package.json', 'utf8'));
const manifest = JSON.parse(execFileSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], { encoding: 'utf8' }));
const thisPackage = manifest.packages.find((item) => item.name === 'migration-commit-witness');
const rustc = execFileSync('rustc', ['--version'], { encoding: 'utf8' });

if (Number(process.versions.node.split('.')[0]) < minimumNode) {
  throw new Error(`Node ${minimumNode}+ is required; found ${process.version}`);
}
if (packageJson.engines?.node !== '>=22') throw new Error('package.json must declare Node 22+');
if (thisPackage?.rust_version !== '1.85') throw new Error('Cargo.toml must declare rust-version = 1.85');
const foundRust = (rustc.match(/rustc (\d+)\.(\d+)\.(\d+)/) ?? []).slice(1).map(Number);
if (foundRust.length !== 3 || foundRust.some((part, index) => part < minimumRust[index] && foundRust.slice(0, index).every((prior, priorIndex) => prior === minimumRust[priorIndex]))) {
  throw new Error(`Rust 1.85+ is required; found ${rustc.trim()}`);
}
