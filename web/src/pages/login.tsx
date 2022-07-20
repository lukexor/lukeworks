import Layout from "components/layout";
import copy from "data/login.json";
import GuestGuard from "guards/GuestGuard";

export default function Login() {
  return (
    <Layout>
      <GuestGuard>
        <h1>{copy.title}</h1>
      </GuestGuard>
    </Layout>
  );
}
