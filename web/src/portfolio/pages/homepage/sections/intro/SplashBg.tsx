import "./SplashBg.css";
import { useEffect, useState } from "react";

const SplashBg = () => {
  const [glitch, setGlitch] = useState(false);
  const [color, setColor] = useState(false);
  const [left, setLeft] = useState(false);

  useEffect(() => {
    let glitchReset: ReturnType<typeof setTimeout>;
    const glitchTimer = setInterval(() => {
      if (Math.random() < 0.3) {
        Math.random() < 0.5 ? setColor(false) : setColor(true);
        Math.random() < 0.5 ? setLeft(true) : setLeft(false);
        setGlitch(true);

        glitchReset = setTimeout(() => {
          setGlitch(false);
        }, 300);
      }
    }, 2000);

    return () => {
      clearInterval(glitchTimer);
      clearTimeout(glitchReset);
    };
  }, []);

  return (
    <div className="splash-bg">
      <div className="bg" />
      <div
        className={[
          "bg-glitch",
          glitch ? "glitching" : "",
          color ? "color" : "",
          left ? "left" : "",
        ]
          .filter(Boolean)
          .join(" ")}
      />
    </div>
  );
};

export default SplashBg;
