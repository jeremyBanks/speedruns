export const inBrowser =
      typeof window === 'object' &&
      typeof document === 'object' &&
      typeof navigator == 'object';

export const fetch =
      inBrowser
      ? window.fetch
      : () => { throw new Error("no fetch() implementation available");

export const url =
      inBrowser