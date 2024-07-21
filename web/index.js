import init from '../dist/blorf.js';

(async ()  => {
  try {
    await init();
  } catch(e) {
    console.error(e);
  }
})();