import './styles.css';

const BUILD_ID = '0.1.1+polish.2';

export function startCommon(): void {
  document.querySelectorAll<HTMLElement>('[data-build]').forEach((node) => { node.textContent = BUILD_ID; });

  const announce = document.getElementById('route-status');
  const heading = document.querySelector<HTMLElement>('main h1');
  if (announce && heading) announce.textContent = heading.textContent ?? '';
  const focusHeading = (): void => {
    if (!heading) return;
    sessionStorage.removeItem('mcw:focus-heading');
    window.requestAnimationFrame(() => heading.focus());
  };
  if (heading && (location.pathname.startsWith('/demo') || sessionStorage.getItem('mcw:focus-heading') === '1')) focusHeading();
  window.addEventListener('pageshow', (event) => {
    if (event.persisted || performance.getEntriesByType('navigation').some((entry) => (entry as PerformanceNavigationTiming).type === 'back_forward')) focusHeading();
  });
  document.querySelectorAll<HTMLAnchorElement>('a[href^="/"]').forEach((link) => {
    link.addEventListener('click', () => sessionStorage.setItem('mcw:focus-heading', '1'));
  });

  if ('serviceWorker' in navigator && location.protocol !== 'file:') {
    window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => undefined));
  }
}

export function copyButtons(): void {
  const status = document.getElementById('copy-status');
  document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach((button) => {
    button.addEventListener('click', async () => {
      const source = document.getElementById(button.dataset.copy ?? '');
      if (!source || !status) return;
      try {
        await navigator.clipboard.writeText(source.textContent ?? '');
        status.textContent = 'Install command copied.';
      } catch {
        status.textContent = 'Copy was blocked. Select the command and copy it.';
      }
    });
  });
}
