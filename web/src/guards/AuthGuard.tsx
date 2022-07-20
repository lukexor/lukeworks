import routes from "data/routes.json";
import useAuth from "hooks/useAuth";
import { useRouter } from "next/router";

export type AuthGuardProps = {
  children?: React.ReactNode;
};

export default function AuthGuard({ children }: AuthGuardProps) {
  const router = useRouter();
  const { isAuthenticated } = useAuth();

  if (!isAuthenticated) {
    router.replace(routes.login.path);
    return null;
  }

  return <>{children}</>;
}
