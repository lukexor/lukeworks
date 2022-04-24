import "./Portfolio.css";
import useMetaTag from "hooks/useMetaTag";
import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import Footer from "./components/footer";
import Header from "./components/header";
import copy from "./data/copy.json";

const { title, description } = copy.Head;

type Props = {
  children: React.ReactNode;
};

const Portfolio = ({ children }: Props) => {
  const [isLoaded, setIsLoaded] = useState(false);
  const { pathname } = useLocation();

  useMetaTag({ title, description });

  useEffect(() => {
    setTimeout(() => {
      setIsLoaded(true);
    }, 100);
  }, []);

  useEffect(() => {
    window.scrollTo(0, 0);

    // Required because of HashLink using ids without a hash
    const scrollIntoView = () => {
      const hash = window.location.hash.substring(1);
      if (!hash) {
        return;
      }
      const el = document.getElementById(hash);
      if (el) {
        window.scrollTo({ top: el.offsetTop });
      }
    };

    setTimeout(scrollIntoView, 300);
  }, [pathname]);

  return (
    <>
      <div className={`fade-enter ${isLoaded ? "fade-enter-active" : ""}`}>
        <Header />
        {children}
        <Footer />
      </div>
    </>
  );
};

export default Portfolio;
