export type QuickMenuAction = "navigate" | "quit" | undefined;

interface MenuActionHandlers {
  onClose: () => void;
  onNavigate: (view: "settings" | "about") => void;
  onQuit: () => Promise<void>;
}

export async function handleQuickMenuAction(
  key: string,
  action: QuickMenuAction,
  handlers: MenuActionHandlers,
): Promise<void> {
  if (action === "navigate") handlers.onNavigate(key as "settings" | "about");
  if (key === "quit") await handlers.onQuit();
  handlers.onClose();
}
