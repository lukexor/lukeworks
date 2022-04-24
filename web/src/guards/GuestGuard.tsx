import useAuth from "hooks/useAuth";
import { Navigate } from "react-router-dom";
import routes from "routes.json";

type Props = {
  children: React.ReactNode;
};

const GuestGuard = ({ children }: Props) => {
  const { isAuthenticated } = useAuth();

  if (isAuthenticated) {
    return <Navigate to={routes.admin.path} replace />;
  }

  return <>{children}</>;
};

export default GuestGuard;
