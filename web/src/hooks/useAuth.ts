import { useState } from "react";

export default function useAuth() {
  const [isAuthenticated] = useState(process.env["DEBUG"] ? true : false);

  return {
    isAuthenticated,
  };
}
