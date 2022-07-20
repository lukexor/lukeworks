import Loading from "components/loading";
import routes from "data/routes.json";
import useAuth from "hooks/useAuth";
import { useRouter } from "next/router";
import { useEffect } from "react";

export type GuestGuardProps = {
  children?: React.ReactNode;
};

export default function GuestGuard({ children }: GuestGuardProps) {
  const router = useRouter();
  const { isAuthenticated } = useAuth();

  useEffect(() => {
    if (router.isReady && isAuthenticated) {
      router.replace(routes.admin.path);
    }
  }, [isAuthenticated, router]);

  if (isAuthenticated) {
    return <Loading />;
  }

  return <>{children}</>;
}
