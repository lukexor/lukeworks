import { useEffect, useRef } from "react";
import s from "./splashBg.module.css";

export default function SplashBg() {
  const glitchRef = useRef<HTMLDivElement>(null);

  const setGlitching = (glitching: boolean) => {
    glitchRef.current?.style.setProperty(
      "--glitch-opacity",
      glitching ? "50%" : "0",
    );
  };

  const setBgColor = (color: string) => {
    glitchRef.current?.style.setProperty("--glitch-color", `var(${color})`);
  };

  const setTransform = (transform: string) => {
    glitchRef.current?.style.setProperty("--glitch-transform", transform);
  };

  useEffect(() => {
    let glitchReset: ReturnType<typeof setTimeout>;
    const glitchTimer = setInterval(() => {
      if (Math.random() < 0.3) {
        setBgColor(
          Math.random() < 0.4 ? "--color-accent-dark" : "--color-secondary",
        );
        setTransform(Math.random() < 0.5 ? "30px" : "-30px");
        setGlitching(true);

        glitchReset = setTimeout(() => {
          setGlitching(false);
        }, 300);
      }
    }, 2000);

    return () => {
      clearInterval(glitchTimer);
      clearTimeout(glitchReset);
    };
  }, []);

  return (
    <div className={s.wrapper}>
      <div className={s.bg} />
      <div ref={glitchRef} className={s.bgGlitch} />
    </div>
  );
}
