import init, { run_web } from './blorf.js';

(async () => {
  try {
    await init();
    run_web();
  } catch(e) {
    console.error(e);
  }
})();