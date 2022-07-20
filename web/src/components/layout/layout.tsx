import clsx from "clsx";
import ErrorBoundary from "components/errorBoundary";
import Footer from "components/footer";
import HashAnchor from "components/hashAnchor";
import Header from "components/header";
import { useEffect, useState } from "react";
import s from "./layout.module.css";

export type LayoutProps = {
  children?: React.ReactNode;
};

export default function Layout({ children }: LayoutProps) {
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setTimeout(() => {
      setIsLoaded(true);
    }, 100);
  }, []);

  useEffect(() => {
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
  }, []);

  return (
    <div className={s.wrapper}>
      <div className={clsx(s.fadeEnter, isLoaded && s.fadeEnterActive)}>
        <div className={s.bodyContent}>
          <Header />
          <HashAnchor id="top" />
          <ErrorBoundary>{children}</ErrorBoundary>
        </div>
        <Footer />
      </div>
    </div>
  );
}
