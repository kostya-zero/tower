"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  DoorOpenIcon,
  LoaderCircleIcon,
  SendHorizonalIcon,
} from "lucide-react";
import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { MessageResponse } from "@/lib/types/message.types";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { AppState } from "@/lib/enums/appstate";
import { toast } from "@/components/Toasts";
import MessageList from "@/components/MessageList";
import DisconnectDialog from "@/components/DisconnectDialog";

const FETCH_DELAY = 2000;

type Props = {
  setAppState: React.Dispatch<React.SetStateAction<AppState>>;
};

export default function ChatPage({ setAppState }: Props) {
  const [sending, setSending] = useState(false);
  const [message, setMessage] = useState("");
  const [messages, setMessages] = useState<MessageResponse[]>([]);
  const {
    scrollAreaRef,
    isAtBottom,
    setIsAtBottom,
    checkIfAtBottom,
    scrollToBottom,
  } = useScrollToBottom();
  const [showDisconnectDialog, setShowDisconnectDialog] = useState(false);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const [isAtBottom, setIsAtBottom] = useState(true);

  const fetchMessages = () => {
    console.log("fetching messages");
    invoke<MessageResponse[]>("fetch_messages")
      .then((r) => {
        console.log(r);
        setMessages((messages) => [...messages, ...r]);
      })
      .catch((e) => {
        toast({
          title: "Failed to fetch messages",
          description: e.toString(),
        });
      });
  };

  useEffect(() => {
    intervalRef.current = setInterval(fetchMessages, FETCH_DELAY);

    inputRef.current!.focus();

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []);

  useEffect(() => {
    if (isAtBottom) {
      setTimeout(() => {
        scrollToBottom();
      }, 10);
    }
  }, [messages, isAtBottom]);

  async function stopFetching() {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }

  async function startFetching() {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }
    intervalRef.current = setInterval(fetchMessages, FETCH_DELAY);
  }

  const handleMessageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMessage(e.target.value);
  };

  async function sendMessage() {
    setSending(true);
    const send_message = message.trim();
    if (send_message === "") {
      setSending(false);
      return;
    }

    setIsAtBottom(true);

    await stopFetching();
    await invoke("send_message", { message: send_message })
      .then(() => {
        setMessage("");
        fetchMessages();
        setTimeout(() => {
          if (inputRef.current) {
            inputRef.current.focus();
          }
        }, 10);
      })
      .catch((e) => {
        toast({
          title: "Failed to send message",
          description: e.toString(),
        });
      })
      .finally(() => {
        setSending(false);
        startFetching();
      });
  }

  async function askForDisconnect() {
    if (sending) {
      return;
    }
    setShowDisconnectDialog(true);
  }

  async function performDisconnect() {
    await stopFetching();
    await invoke("disconnect");
    setAppState(AppState.Main);
  }

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !sending) {
      sendMessage();
    }
  };

  return (
    <>
      <main className="flex h-screen shrink-0 grow-0 flex-col justify-between">
        <MessageList
          ref={scrollAreaRef}
          messages={messages}
          onScroll={checkIfAtBottom}
        />
        <div className="flex flex-row gap-2 border-t border-neutral-800 p-2">
          <Button
            size="icon"
            variant={"outline"}
            className="cursor-pointer"
            onClick={() => askForDisconnect()}
          >
            <DoorOpenIcon />
          </Button>
          <Input
            placeholder="Your message here..."
            ref={inputRef}
            disabled={sending}
            value={message}
            onChange={handleMessageChange}
            onKeyUp={handleKeyPress}
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
      <DisconnectDialog
        open={showDisconnectDialog}
        onOpenChange={setShowDisconnectDialog}
        onConfirm={performDisconnect}
      />
    </>
  );
}
