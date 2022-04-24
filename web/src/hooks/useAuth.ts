import { useState } from "react";

const useAuth = () => {
  const [isAuthenticated] = useState(false);

  return {
    isAuthenticated,
  };
};

export default useAuth;
