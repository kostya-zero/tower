"use client";

import BadMessageSection from "@/components/BadMessageSection";
import MessageSection from "@/components/MessageSection";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  DoorOpenIcon,
  LoaderCircleIcon,
  SendHorizonalIcon,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { AppState } from "@/app/page";
import { invoke } from "@tauri-apps/api/core";
import { MessageResponse } from "@/lib/types/message.types";

type Props = {
  setAppState: React.Dispatch<React.SetStateAction<AppState>>;
};

type SendResult = {
  success: boolean;
  message: string;
};

export default function ChatPage({ setAppState }: Props) {
  const [sending, setSending] = useState(false);
  const [message, setMessage] = useState("");
  const [messages, setMessages] = useState<MessageResponse[]>([]);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    const fetchMessages = () => {
      console.log("fetching messages");
      invoke<MessageResponse[]>("fetch_messages")
        .then((r) => {
          console.log(r);
          setMessages((messages) => [...messages, ...r]);
        })
        .catch((e) => {
          console.error("Error fetching messages:", e);
        });
    };

    intervalRef.current = setInterval(fetchMessages, 5000);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []);

  async function stopFetchingMessages() {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }

  const handleMessageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMessage(e.target.value);
  };

  async function sendMessage() {
    setSending(true);
    const send_message = message.trim();
    if (send_message === "") return;

    await invoke<SendResult>("send_message", { message: send_message })
      .then((r) => {
        if (r.success) {
          setMessage("");
        } else {
          console.error("Failed to send message:", r.message);
        }
      })
      .catch((e) => {
        console.error("Error sending message:", e);
      })
      .finally(() => setSending(false));
  }

  async function performDisconnect() {
    await stopFetchingMessages();
    await invoke("disconnect");
    setAppState(AppState.Main);
  }

  return (
    <main className="flex flex-col grow-0 justify-between shrink-0 h-screen">
      <ScrollArea
        className="flex-grow w-full overflow-y-auto"
        scrollHideDelay={400}
      >
        {messages.map((message, index) =>
          message.message ? (
            <MessageSection message={message.message} key={index} />
          ) : (
            <BadMessageSection rawString={message.raw_string} key={index} />
          ),
        )}
      </ScrollArea>
      <div className="flex flex-row gap-2 p-2 border-t border-neutral-800">
        <Button
          size="icon"
          variant={"outline"}
          className="cursor-pointer"
          onClick={() => performDisconnect()}
        >
          <DoorOpenIcon />
        </Button>
        <Input
          placeholder="Your message here..."
          disabled={sending}
          value={message}
          onChange={handleMessageChange}
        />
        <Button
          size="icon"
          className="cursor-pointer"
          onClick={sendMessage}
          disabled={sending}
        >
          {sending ? (
            <LoaderCircleIcon className="animate-spin" />
          ) : (
            <SendHorizonalIcon />
          )}
        </Button>
      </div>
    </main>
  );
}
