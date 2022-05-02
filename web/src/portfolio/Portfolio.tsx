import "./Portfolio.css";
import ErrorBoundary from "ErrorBoundary";
import useMetaTag from "hooks/useMetaTag";
import useOnScreen from "hooks/useOnScreen";
import { lazy, Suspense, useEffect, useLayoutEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import HashAnchor from "./components/HashAnchor";
import Header from "./components/header";
import copy from "./data/copy.json";

const { title, description } = copy.Head;

const Footer = lazy(() => import("./components/footer"));

type Props = {
  children: React.ReactNode;
};

const Portfolio = ({ children }: Props) => {
  const [isLoaded, setIsLoaded] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();
  const [footerRef, isFooterVisible] = useOnScreen<HTMLDivElement>();

  useMetaTag({ title, description });

  useEffect(() => {
    setTimeout(() => {
      setIsLoaded(true);
    }, 100);
  }, []);

  useLayoutEffect(() => {
    // Required because of HashAnchor using ids without a hash
    const scrollIntoView = () => {
      const hash = location.hash.substring(1);
      if (!hash) {
        window.scrollTo(0, 0);
        return;
      }
      const el = document.getElementById(hash);
      if (el) {
        window.scrollTo({ top: el.offsetTop, behavior: "smooth" });
      }
    };
    setTimeout(scrollIntoView, 100);
  }, [location.hash, location.pathname]);

  return (
    <>
      <div className={`fade-enter ${isLoaded ? "fade-enter-active" : ""}`}>
        <div className="body-content">
          <Header />
          <HashAnchor id="top" />
          <ErrorBoundary key={location.pathname} navigate={navigate}>
            {children}
          </ErrorBoundary>
        </div>
        <div ref={footerRef} className="lazy">
          <Suspense fallback={null}>
            {isFooterVisible ? <Footer /> : null}
          </Suspense>
        </div>
      </div>
    </>
  );
};

export default Portfolio;
