// This module wraps all external interfaces so they can be subbed out for different platforms.
// This implementation is for Node.

export const document = undefined;

export const window = undefined;

import url from 'url';
const URL = url.URL;
export {URL};

import fetch from 'node-fetch';
export {fetch};
