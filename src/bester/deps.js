// This module wraps all external interfaces so they can be subbed out for different platforms.
// This implementation is for the browser.

export const document = window.document;

// TODO: eliminate reference to window, since it's way too broad.
const window_ = window;
export {window_ as window};

export const fetch = window.fetch;

export const URL = window.URL;
