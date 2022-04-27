import "./LukeWorks.css";
import legacyRoutes from "legacyRoutes.json";
import { lazy, Suspense } from "react";
import { Navigate, Outlet, Route, Routes, useNavigate } from "react-router-dom";
import routes from "routes.json";
import ErrorBoundary from "./ErrorBoundary";

const { resume, login, admin, post, portfolio } = routes;

const GuestGuard = lazy(() => import("guards/GuestGuard"));
const AuthGuard = lazy(() => import("guards/AuthGuard"));
const NotFound = lazy(() => import("./NotFound"));

const Portfolio = lazy(() => import("portfolio"));
const Homepage = lazy(() => import("portfolio/pages/homepage"));
const Post = lazy(() => import("portfolio/pages/post"));
const Resume = lazy(() => import("resume/Resume"));
const Admin = lazy(() => import("admin/Admin"));
const Login = lazy(() => import("admin/Login"));

const LukeWorks = () => {
  const navigate = useNavigate();

  return (
    <ErrorBoundary navigate={navigate}>
      <Suspense fallback={null}>
        <Routes>
          <Route
            element={
              <Portfolio>
                <Suspense fallback={null}>
                  <Outlet />
                </Suspense>
              </Portfolio>
            }
          >
            {/* TODO handle search */}
            {Object.entries(legacyRoutes).map(([legacyRoute, newRoute]) => (
              <Route
                key={legacyRoute}
                path={legacyRoute}
                element={<Navigate to={newRoute} replace />}
              />
            ))}
            <Route path={post.path} element={<Post />} />
            <Route path={portfolio.path} element={<Homepage />} />
          </Route>
          <Route path={resume.path} element={<Resume />} />
          <Route
            path={login.path}
            element={
              <GuestGuard>
                <Login />
              </GuestGuard>
            }
          />
          <Route
            path={admin.path}
            element={
              <AuthGuard>
                <Admin />
              </AuthGuard>
            }
          />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Suspense>
    </ErrorBoundary>
  );
};

export default LukeWorks;
