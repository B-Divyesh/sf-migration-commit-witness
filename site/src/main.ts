import './styles.css';
import {
  LICENSE_KEY,
  VERDICT_KEY,
  isFreshVerdict,
  parseCachedVerdict,
  tokenFingerprint,
  verificationUrl,
  type CachedVerdict,
  type LicenseVerdict,
} from './license';

type DemoStage = {
  label: string;
  verdict: string;
  verdictClass: string;
  schema: string;
  rows: string;
  next: string;
  output: string;
};

const stages: DemoStage[] = [
  {
    label: 'BASELINE / captured', verdict: 'Not checked', verdictClass: 'verdict-neutral', schema: '0 / 2', rows: '12', next: 'Run command',
    output: '$ mcw witness --confirm-test-database\nsnapshot.before.schema_objects = 0\nsnapshot.before.account_rows = 12\nsqlite.quick_check = "ok"',
  },
  {
    label: 'MIGRATION / reported', verdict: 'Exit 0', verdictClass: 'verdict-success', schema: '1 / 2', rows: '12', next: 'Inspect after',
    output: '$ sh ./migrations/up.sh\nframework.status = "success"\ncommand.exit_code = 0\ncommand.reported_success = true',
  },
  {
    label: 'AFTER COMMIT / witnessed', verdict: 'Witness failed', verdictClass: 'verdict-fail', schema: '1 / 2', rows: '12', next: 'Exercise rollback',
    output: 'snapshot.after.schema_objects = 1\nexpected.schema_objects = 2\nassertion = "FAIL"\nreason = "partial commit detected"',
  },
  {
    label: 'ROLLBACK / exercised', verdict: 'Claim proved', verdictClass: 'verdict-success', schema: '0 / 2', rows: '12', next: 'Replay fixture',
    output: '$ sh ./migrations/down.sh\nrollback.exit_code = 0\nsnapshot.rollback.schema_objects = 0\nrollback.matches_before = true\nartifact.signature = "HMAC-SHA256"',
  },
];

let activeStage = 0;
const tabs = [...document.querySelectorAll<HTMLButtonElement>('[role="tab"]')];
const panel = byId('stage-panel');
const output = byId('stage-output');
const label = byId('stage-label');
const verdict = byId('demo-verdict');
const schema = byId('visual-schema');
const rows = byId('visual-rows');
const previous = byId<HTMLButtonElement>('demo-prev');
const next = byId<HTMLButtonElement>('demo-next');

function renderStage(index: number, focusPanel = false): void {
  activeStage = (index + stages.length) % stages.length;
  const stage = stages[activeStage];
  tabs.forEach((tab, tabIndex) => {
    const selected = tabIndex === activeStage;
    tab.setAttribute('aria-selected', String(selected));
    tab.tabIndex = selected ? 0 : -1;
  });
  panel.setAttribute('aria-labelledby', tabs[activeStage].id);
  label.textContent = stage.label;
  verdict.textContent = stage.verdict;
  verdict.className = `verdict ${stage.verdictClass}`;
  output.textContent = stage.output;
  schema.textContent = stage.schema;
  rows.textContent = stage.rows;
  previous.disabled = activeStage === 0;
  next.innerHTML = `${stage.next} <span aria-hidden="true">${activeStage === stages.length - 1 ? '↻' : '→'}</span>`;
  if (focusPanel) panel.focus();
}

tabs.forEach((tab, index) => {
  tab.addEventListener('click', () => renderStage(index));
  tab.addEventListener('keydown', (event) => {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const target = event.key === 'Home' ? 0 : event.key === 'End' ? tabs.length - 1 : (index + (event.key === 'ArrowRight' ? 1 : -1) + tabs.length) % tabs.length;
    renderStage(target);
    tabs[target].focus();
  });
});
previous.addEventListener('click', () => renderStage(activeStage - 1, true));
next.addEventListener('click', () => renderStage(activeStage === stages.length - 1 ? 0 : activeStage + 1, true));

const copyStatus = byId('copy-status');
document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach((button) => {
  button.addEventListener('click', async () => {
    const source = byId(button.dataset.copy ?? '');
    try {
      await navigator.clipboard.writeText(source.textContent ?? '');
      copyStatus.textContent = `${button.textContent} complete.`;
      const original = button.textContent;
      button.textContent = 'Copied';
      window.setTimeout(() => { button.textContent = original; }, 1800);
    } catch {
      copyStatus.textContent = 'Clipboard access was blocked. Select the command text to copy it.';
    }
  });
});

const offlineBar = byId('offline-bar');
const OFFLINE_SESSION_KEY = 'mcw:offline';
function showOffline(): void {
  sessionStorage.setItem(OFFLINE_SESSION_KEY, '1');
  offlineBar.hidden = false;
}
async function renderConnectivity(): Promise<void> {
  if (sessionStorage.getItem(OFFLINE_SESSION_KEY) === '1') {
    offlineBar.hidden = false;
    return;
  }
  let online = navigator.onLine;
  if (online) {
    try {
      const response = await fetch('/favicon.svg', { method: 'HEAD', cache: 'no-store' });
      online = response.ok;
    } catch {
      online = false;
    }
  }
  offlineBar.hidden = online;
  if (online) sessionStorage.removeItem(OFFLINE_SESSION_KEY);
  else sessionStorage.setItem(OFFLINE_SESSION_KEY, '1');
}
window.addEventListener('online', () => { sessionStorage.removeItem(OFFLINE_SESSION_KEY); void renderConnectivity(); void reconcileStoredLicense(); });
window.addEventListener('offline', showOffline);
void renderConnectivity();

const form = byId<HTMLFormElement>('license-form');
const tokenInput = byId<HTMLInputElement>('license-token');
const licenseStatus = byId('license-status');
const kitButton = byId<HTMLButtonElement>('download-kit');

function setLicenseState(state: 'locked' | 'loading' | 'unlocked' | 'error', message: string): void {
  kitButton.disabled = state !== 'unlocked';
  licenseStatus.textContent = message;
  licenseStatus.dataset.state = state === 'unlocked' ? 'success' : state === 'error' ? 'error' : '';
  tokenInput.setAttribute('aria-invalid', String(state === 'error'));
}

async function verifyLicense(token: string, force = false): Promise<void> {
  const cached = parseCachedVerdict(localStorage.getItem(VERDICT_KEY));
  if (!force && cached && isFreshVerdict(cached, token)) {
    if (cached.valid) setLicenseState('unlocked', 'License active. The rollout kit is ready.');
    else setLicenseState('error', `License no longer active (${cached.reason.replace('_', ' ')}). You can purchase a new license above.`);
    return;
  }
  if (!navigator.onLine) {
    if (cached?.valid && cached.tokenFingerprint === tokenFingerprint(token)) setLicenseState('unlocked', 'Offline — using the last valid license check.');
    else setLicenseState('locked', 'Offline — connect once to verify this license.');
    return;
  }
  setLicenseState('loading', 'Checking this license…');
  try {
    const response = await fetch(verificationUrl(token), { headers: { Accept: 'application/json' } });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const verdict = await response.json() as LicenseVerdict;
    const stored: CachedVerdict = { ...verdict, checkedAt: Date.now(), tokenFingerprint: tokenFingerprint(token) };
    localStorage.setItem(VERDICT_KEY, JSON.stringify(stored));
    if (verdict.valid) setLicenseState('unlocked', 'License active. The rollout kit is ready.');
    else setLicenseState('error', `License no longer active (${verdict.reason.replace('_', ' ')}). You can purchase a new license above.`);
  } catch {
    if (cached?.valid && cached.tokenFingerprint === tokenFingerprint(token)) setLicenseState('unlocked', 'Verification is unavailable — using the last valid check.');
    else setLicenseState('error', 'Could not reach license verification. Check your connection and try again.');
  }
}

async function reconcileStoredLicense(): Promise<void> {
  const token = localStorage.getItem(LICENSE_KEY)?.trim();
  if (!token) return;
  tokenInput.value = token;
  const cached = parseCachedVerdict(localStorage.getItem(VERDICT_KEY));
  if (cached?.valid && cached.tokenFingerprint === tokenFingerprint(token)) setLicenseState('unlocked', 'License active from the last verified check.');
  await verifyLicense(token);
}

form.addEventListener('submit', (event) => {
  event.preventDefault();
  const token = tokenInput.value.trim();
  if (!token) {
    setLicenseState('error', 'Paste the full license token, then verify again.');
    tokenInput.focus();
    return;
  }
  localStorage.setItem(LICENSE_KEY, token);
  void verifyLicense(token, true);
});

kitButton.addEventListener('click', () => {
  const kit = `# Migration Commit Witness — team rollout kit\n\nLicensed team template for v0.x\n\n## Review owners\n- Migration author:\n- Database reviewer:\n- Release owner:\n\n## Required CI policy\n- [ ] Disposable database URL supplied by CI\n- [ ] Dialect named in mcw.toml\n- [ ] Commit invariants cover schema and critical data\n- [ ] Explicit rollback command reviewed\n- [ ] --exercise-rollback enabled before production merge\n- [ ] witness.json and witness.md retained as artifacts\n- [ ] Signature verified in the release job\n\n## Exception record\n- Invariant omitted:\n- Reason:\n- Owner:\n- Expiry:\n`;
  const anchor = document.createElement('a');
  anchor.href = URL.createObjectURL(new Blob([kit], { type: 'text/markdown' }));
  anchor.download = 'mcw-team-rollout-kit.md';
  anchor.click();
  URL.revokeObjectURL(anchor.href);
  setLicenseState('unlocked', 'Team rollout kit downloaded.');
});

const returnedToken = new URLSearchParams(location.search).get('license')?.trim();
if (returnedToken) {
  localStorage.setItem(LICENSE_KEY, returnedToken);
  history.replaceState({}, '', `${location.pathname}${location.hash}`);
}
void reconcileStoredLicense();

if ('serviceWorker' in navigator && location.protocol !== 'file:') {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => undefined));
}

function byId<T extends HTMLElement = HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Missing #${id}`);
  return element as T;
}
