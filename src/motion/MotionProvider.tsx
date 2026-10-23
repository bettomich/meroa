import { createContext, useContext, useLayoutEffect, useMemo, useState, useSyncExternalStore, type ReactNode } from "react";
import { readAnimationsEnabled, saveAnimationsEnabled } from "../preferences";

const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");
const subscribe = (notify: () => void) => {
  reducedMotionQuery.addEventListener("change", notify);
  return () => reducedMotionQuery.removeEventListener("change", notify);
};
const getReducedMotion = () => reducedMotionQuery.matches;

interface MotionSettings {
  animationsEnabled: boolean;
  motionEnabled: boolean;
  reducedMotion: boolean;
  setAnimationsEnabled: (enabled: boolean) => void;
}

const MotionContext = createContext<MotionSettings | null>(null);

export function MotionProvider({ children }: { children: ReactNode }) {
  const [animationsEnabled, setEnabled] = useState(readAnimationsEnabled);
  const reducedMotion = useSyncExternalStore(subscribe, getReducedMotion);
  const motionEnabled = animationsEnabled && !reducedMotion;

  useLayoutEffect(() => {
    // A single gate cancels CSS animations/transitions before paint. No animation
    // timers, requestAnimationFrame loops, or spring calculations run in React.
    document.documentElement.dataset.motion = motionEnabled ? "on" : "off";
    return () => { delete document.documentElement.dataset.motion; };
  }, [motionEnabled]);

  const value = useMemo<MotionSettings>(() => ({
    animationsEnabled,
    motionEnabled,
    reducedMotion,
    setAnimationsEnabled: (enabled) => {
      setEnabled(enabled);
      saveAnimationsEnabled(enabled);
    },
  }), [animationsEnabled, motionEnabled, reducedMotion]);

  return <MotionContext.Provider value={value}>{children}</MotionContext.Provider>;
}

export function useMotion(): MotionSettings {
  const settings = useContext(MotionContext);
  if (!settings) throw new Error("useMotion must be used within MotionProvider");
  return settings;
}
