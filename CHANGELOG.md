# Changelog

All notable changes use [Semantic Versioning](https://semver.org/).

## 0.1.1 — 2026-08-28

- Fix PostgreSQL URI handling and redact connection strings from query errors.
- Fail witnesses when baseline or rollback scalar queries error.
- Validate requested rollback configuration before touching the database.
- Cache valid and invalid license verdicts for the documented 24-hour window.
- Ship Azure-native cache/security headers, reliable service-worker updates,
  and 44 px inline touch targets.

## 0.1.0 — 2026-08-28

- Add SQLite and PostgreSQL commit witnesses with scalar invariants.
- Add explicit, opt-in rollback exercise and dialect health probes.
- Add HMAC-SHA256 signed JSON/Markdown output and offline verification.
- Add production guards, CI exit codes, and machine-readable stdout.
