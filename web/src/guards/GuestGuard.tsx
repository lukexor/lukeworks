import routes from "data/routes.json";
import useAuth from "hooks/useAuth";
import { useRouter } from "next/router";

export type GuestGuardProps = {
  children?: React.ReactNode;
};

export default function GuestGuard({ children }: GuestGuardProps) {
  const router = useRouter();
  const { isAuthenticated } = useAuth();

  if (isAuthenticated) {
    router.replace(routes.admin.path);
    return null;
  }

  return <>{children}</>;
}
