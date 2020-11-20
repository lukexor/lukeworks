import { useCallback, useEffect, useState } from "react";

// Credit: https://usehooks.com/useAsync/
const useAsync = (asyncFunc, immediate = true) => {
  const [pending, setPending] = useState(false);
  const [result, setResult] = useState(null);
  const [error, setError] = useState(null);

  // The execute function wraps asyncFunction and
  // handles setting state for pending, value, and error.
  // useCallback ensures the below useEffect is not called
  // on every render, but only if asyncFunction changes.
  const execute = useCallback(() => {
    setPending(true);
    setResult(null);
    setError(null);
    return asyncFunc()
      .then((response) => setResult(response))
      .catch((error) => setError(error))
      .finally(() => setPending(false));
  }, [asyncFunc]);

  // Call execute if we want to fire it right away.
  // Otherwise execute can be called later, such as
  // in an onClick handler.
  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [execute, immediate]);

  return { execute, pending, result, error };
};

export default useAsync;
