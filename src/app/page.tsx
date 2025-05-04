"use client";

import { useState } from "react";
import MainPage from "./Main";
import ChatPage from "./Chat";

enum AppState {
  Main,
  Chat,
}

export default function App() {
  const [state, setState] = useState<AppState>(AppState.Main);
  return (
    <>
      {state === AppState.Main && <MainPage />}
      {state === AppState.Chat && <ChatPage />}
    </>
  );
}
