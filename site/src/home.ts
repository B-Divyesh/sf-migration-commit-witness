import { copyButtons, startCommon } from './common';

if (new URLSearchParams(location.search).get('demo') === '1') {
  location.replace('/demo/');
} else {
  startCommon();
  copyButtons();
}
