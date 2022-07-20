import Layout from "components/layout";
import AuthGuard from "guards/AuthGuard";

export default function Admin() {
  return (
    <Layout>
      <AuthGuard></AuthGuard>
    </Layout>
  );
}
