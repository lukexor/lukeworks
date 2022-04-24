import useAuth from "hooks/useAuth";
import { Navigate, useLocation } from "react-router-dom";
import routes from "routes.json";

type Props = {
  children: React.ReactNode;
};

const AuthGuard = ({ children }: Props) => {
  const location = useLocation();
  const { isAuthenticated } = useAuth();

  if (!isAuthenticated) {
    return (
      <Navigate to={routes.login.path} replace state={{ from: location }} />
    );
  }

  return <>{children}</>;
};

export default AuthGuard;
