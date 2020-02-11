import { useEffect } from "react";
import NProgress from "nprogress";

const minDelay = 200;

// GLOBAL STATE!
let progressStack = 0;
let startTimeout: undefined | NodeJS.Timeout;

export const inc = () => {
  progressStack += 1;
  if (!startTimeout) {
    if (progressStack <= 1) {
      startTimeout = setTimeout(() => {
        NProgress.start();
        startTimeout = undefined;
      }, minDelay);
    } else {
      NProgress.inc();
    }
  }
};

export const dec = () => {
  progressStack -= 1;
  if (progressStack < 0) {
    // tslint:disable-next-line:no-console
    console.warn("progressStack < 0");
    progressStack = 0;
  }
  if (!startTimeout) {
    if (progressStack <= 0) {
      NProgress.done();
    } else {
      NProgress.inc();
    }
  } else {
    if (progressStack <= 0) {
      clearTimeout(startTimeout);
      startTimeout = undefined;
    }
  }
};

const useNprogress = (loading: boolean) => {
  useEffect(() => {
    if (loading) {
      inc();

      return dec;
    }
  }, [loading]);
};

export default useNprogress;
