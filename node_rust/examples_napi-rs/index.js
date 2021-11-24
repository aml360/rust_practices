'use strict';
// @ts-check

const { loadBinding } = require('@node-rs/helper');
/**
 * __dirname means load native addon from current dir
 * 'aml360_napi-rs_test' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `aml360_napi-rs_test.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `aml360_napi-rs_test-[PLATFORM]`
 * @type {{amliPromise: (seconds: number, text: string) => Promise<string> }}
 */
const bindings = loadBinding(__dirname, 'aml360_napi-rs_test', 'aml360_napi-rs_test');

/**
 * @param {number} seconds Number to seconds to complete the promise
 * @param {string} text Any text that will be returned from rust
 * @returns {Promise} promise that will resolve as the same string in args[1]
 */
const amliPromise = bindings.amliPromise;

(async () => {
  const returnedText = amliPromise(1000, 'holaQTal');
  try {
    //This should return an error because 'rs rocks' is an invalid text
    await amliPromise(1000, 'rs rocks');
  } catch (error) {
    console.log('Error works as expected', error);
  }
  console.log(await returnedText);
})();
