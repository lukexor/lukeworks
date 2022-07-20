import { useCallback, useEffect, useState } from "react";

export enum AsyncStatus {
  idle,
  pending,
  success,
  error,
}

export type UseAsyncResult<T, E> = {
  execute: () => Promise<T>;
  status: AsyncStatus;
  result: null | T;
  error: null | E;
};

// Credit: https://usehooks.com/useAsync/
export default function useAsync<T, E = string>(
  asyncFunc: () => Promise<T>,
  immediate = true,
): UseAsyncResult<T, E> {
  const [status, setStatus] = useState(AsyncStatus.idle);
  const [result, setResult] = useState<null | T>(null);
  const [error, setError] = useState<null | E>(null);

  const execute = useCallback(async () => {
    setStatus(AsyncStatus.pending);
    setResult(null);
    setError(null);
    return asyncFunc()
      .then((response) => {
        setResult(response);
        setStatus(AsyncStatus.success);
        return response;
      })
      .catch((error) => {
        setError(error);
        setStatus(AsyncStatus.error);
        return error;
      });
  }, [asyncFunc]);

  // Call execute if we want to fire it right away.
  // Otherwise execute can be called later, such as
  // in an onClick handler.
  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [execute, immediate]);

  return { execute, status, result, error };
}
