// This module wraps all external interfaces so they can be subbed out for different platforms.
// This implementation is for Node.
import url from 'URL';

import fetch from 'node-fetch';


// Will be null/undefined if running in a non-browser environment:
export const document = undefined;
export const window = undefined;

// Will have an partial/limited replacement in a non-browser environment:
export {fetch};

// Will behave ~identically in a non-browser environment:
export const URL = url.URL;
