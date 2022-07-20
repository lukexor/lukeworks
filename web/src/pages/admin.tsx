import AuthGuard from "guards/AuthGuard";

export default function Admin() {
  return <AuthGuard></AuthGuard>;
}
