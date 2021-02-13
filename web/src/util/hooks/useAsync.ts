import { useCallback, useEffect, useState } from "react";

enum AsyncStatus {
  idle,
  pending,
  success,
  error,
}
type UseAsyncResult<T, E> = {
  execute: () => Promise<T>;
  status: AsyncStatus;
  result: Maybe<T>;
  error: Maybe<E>;
};

// Credit: https://usehooks.com/useAsync/
const useAsync = <T, E = string>(
  asyncFunc: () => Promise<T>,
  immediate = true
): UseAsyncResult<T, E> => {
  const [status, setStatus] = useState(AsyncStatus.idle);
  const [result, setResult] = useState<Maybe<T>>(null);
  const [error, setError] = useState<Maybe<E>>(null);

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
};

export default useAsync;
