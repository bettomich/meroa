import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { I18nProvider } from "./i18n/I18nProvider";
import { MotionProvider } from "./motion/MotionProvider";
import "./styles/global.css";
import "./styles/motion.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <I18nProvider>
      <MotionProvider><App /></MotionProvider>
    </I18nProvider>
  </StrictMode>,
);
