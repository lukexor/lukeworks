import { useState } from "react";

const useAuth = () => {
  const [isAuthenticated] = useState(
    process.env["REACT_APP_DEBUG"] ? true : false
  );

  return {
    isAuthenticated,
  };
};

export default useAuth;
