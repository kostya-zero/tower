"use client";

import { useState } from "react";
import MainPage from "./Main";
import ChatPage from "./Chat";
import { AppState } from "@/lib/enums/appstate";

export default function App() {
  const [state, setState] = useState<AppState>(AppState.Main);
  return (
    <>
      {state === AppState.Main && <MainPage setAppState={setState} />}
      {state === AppState.Chat && <ChatPage setAppState={setState} />}
    </>
  );
}
