import init, { web } from './blorf.js';

(async () => {
  try {
    await init();
    web();
  } catch(e) {
    console.error(e);
  }
})();