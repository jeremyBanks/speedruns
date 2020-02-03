import { useState, useEffect } from "react";

export const useDebounced = <T>(value: T, interval: number) => {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const timeout = setTimeout(() => void setDebouncedValue(value), interval);
    return () => void clearTimeout(timeout);
  }, [value, interval]);

  return debouncedValue;
};
