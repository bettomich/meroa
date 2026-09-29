export async function requestNativeQuit(
  invokeCommand: (command: string) => Promise<unknown>,
): Promise<void> {
  await invokeCommand("quit_application_command");
}
