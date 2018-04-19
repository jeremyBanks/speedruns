// A custom node module loader to allow us to import our client-side modules
// using the same paths as we do on the client, for easy server-side reuse.

import url from 'url';
import path from 'path';
import process from 'process';
import Module from 'module';

const builtins = Module.builtinModules;

const baseURL = new url.URL('file://');
baseURL.pathname = `${process.cwd()}/`;

export function resolve(specifier, parentModuleURL = baseURL, defaultResolve) {
  // built-in modules
  if (builtins.includes(specifier)) {
    return {
      url: specifier,
      format: 'builtin'
    };
  }

  // node_modules
  if (/^\.{0,2}[/]/.test(specifier) !== true && !specifier.startsWith('file:')) {
    return defaultResolve(specifier, parentModuleURL);
  }

  // local modules
  const resolved = new url.URL(specifier.replace(/^\/assets\//, './src/'), parentModuleURL);
  const ext = path.extname(resolved.pathname);
  if (ext !== '.js') {
    throw new Error(
      `Cannot load file with non-JavaScript file extension ${ext}.`);
  }
  return {
    url: resolved.href,
    format: 'esm'
  };
}
