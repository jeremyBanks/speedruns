import { useEffect } from "react";
import NProgress from "nprogress";

const startDelay = 10;
const doneDelay = 10;

// GLOBAL STATE!
let progressStack = 0;
let startTimeout: undefined | NodeJS.Timeout;
let doneTimeout: undefined | NodeJS.Timeout;

NProgress.configure({ trickle: false });

export const inc = () => {
  progressStack += 1;
  if (!startTimeout) {
    if (progressStack <= 1) {
      if (doneTimeout) {
        clearTimeout(doneTimeout);
      } else {
        startTimeout = setTimeout(() => {
          NProgress.start();
          startTimeout = undefined;
        }, startDelay);
      }
    } else {
      NProgress.inc();
    }
  } else {
    NProgress.inc();
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
      doneTimeout = setTimeout(() => {
        NProgress.done();
        doneTimeout = undefined;
      }, doneDelay);
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

const useProgressIndicator = (loading: boolean) => {
  useEffect(() => {
    if (loading) {
      inc();

      return dec;
    }
  }, [loading]);
};

export default useProgressIndicator;
