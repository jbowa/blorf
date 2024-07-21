import init from './blorf.js';

(async ()  => {
  try {
    await init();
  } catch(e) {
    console.error(e);
  }
})();