import { startCommon } from './common';

type Stage = { id: string; label: string; verdict: string; status: string; schema: string; rows: string; output: string };
type DemoRecord = { generated_by: string; command: string; stages: Stage[] };

startCommon();

const namespace = 'demo:mcw:';
const panel = byId('stage-panel');
const tabs = [...document.querySelectorAll<HTMLButtonElement>('[role="tab"]')];
const status = byId('demo-status');
let record: DemoRecord | undefined;
let active = 2;

function render(index: number, focusPanel = false): void {
  if (!record) return;
  active = (index + record.stages.length) % record.stages.length;
  sessionStorage.setItem(`${namespace}stage`, String(active));
  const stage = record.stages[active];
  tabs.forEach((tab, tabIndex) => {
    const selected = tabIndex === active;
    tab.setAttribute('aria-selected', String(selected));
    tab.tabIndex = selected ? 0 : -1;
  });
  panel.setAttribute('aria-labelledby', tabs[active].id);
  byId('stage-label').textContent = stage.label;
  byId('stage-verdict').textContent = stage.verdict;
  byId('stage-verdict').dataset.status = stage.status;
  byId('stage-schema').textContent = stage.schema;
  byId('stage-rows').textContent = stage.rows;
  byId('stage-output').textContent = stage.output;
  status.textContent = `${stage.label}. ${stage.verdict}.`;
  byId<HTMLButtonElement>('demo-prev').disabled = active === 0;
  byId('demo-next').textContent = active === record.stages.length - 1 ? 'Replay sample' : 'Show next observation';
  if (focusPanel) panel.focus();
}

async function load(): Promise<void> {
  try {
    const response = await fetch('/demo-record.json');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    record = await response.json() as DemoRecord;
    const raw = sessionStorage.getItem(`${namespace}stage`);
    const stored = raw === null ? Number.NaN : Number(raw);
    render(Number.isInteger(stored) && stored >= 0 ? stored : 2);
  } catch {
    byId('demo-error').hidden = false;
    byId('demo-shell').hidden = true;
  }
}

tabs.forEach((tab, index) => {
  tab.addEventListener('click', () => render(index));
  tab.addEventListener('keydown', (event) => {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const target = event.key === 'Home' ? 0 : event.key === 'End' ? tabs.length - 1 : (index + (event.key === 'ArrowRight' ? 1 : -1) + tabs.length) % tabs.length;
    render(target);
    tabs[target].focus();
  });
});
byId('demo-prev').addEventListener('click', () => render(active - 1, true));
byId('demo-next').addEventListener('click', () => render(record && active === record.stages.length - 1 ? 0 : active + 1, true));
byId('reset-demo').addEventListener('click', () => {
  Object.keys(sessionStorage).filter((key) => key.startsWith(namespace)).forEach((key) => sessionStorage.removeItem(key));
  render(2, true);
  status.textContent = 'Demo reset to the detected partial commit.';
});
byId('start-real').addEventListener('click', () => {
  Object.keys(sessionStorage).filter((key) => key.startsWith(namespace)).forEach((key) => sessionStorage.removeItem(key));
});

void load();

function byId<T extends HTMLElement = HTMLElement>(id: string): T {
  const node = document.getElementById(id);
  if (!node) throw new Error(`Missing #${id}`);
  return node as T;
}
