"use client";

import { useState } from "react";
import MainPage from "./Main";
import ChatPage from "./Chat";

export enum AppState {
  Main,
  Chat,
}

export default function App() {
  const [state, setState] = useState<AppState>(AppState.Main);
  return (
    <>
      {state === AppState.Main && <MainPage setAppState={setState} />}
      {state === AppState.Chat && <ChatPage setAppState={setState} />}
    </>
  );
}
