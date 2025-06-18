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
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

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
  const [showDisconnectDialog, setShowDisconnectDialog] = useState(false);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const [isAtBottom, setIsAtBottom] = useState(true);
  const delay = 2500;

  const checkIfAtBottom = () => {
    const scrollContainer = scrollAreaRef.current?.querySelector(
      "[data-radix-scroll-area-viewport]",
    );
    if (scrollContainer) {
      const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
      const threshold = 10; // Small threshold to account for floating point precision
      const atBottom = scrollTop + clientHeight >= scrollHeight - threshold;
      setIsAtBottom(atBottom);
    }
  };

  const scrollToBottom = () => {
    const scrollContainer = scrollAreaRef.current?.querySelector(
      "[data-radix-scroll-area-viewport]",
    );
    if (scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  };

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

  useEffect(() => {
    intervalRef.current = setInterval(fetchMessages, delay);

    inputRef.current.focus();

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

  async function stopFetchingMessages() {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }

  async function startFetchingMessages() {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }
    intervalRef.current = setInterval(fetchMessages, delay);
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

    await stopFetchingMessages();
    await invoke<SendResult>("send_message", { message: send_message })
      .then((r) => {
        if (r.success) {
          setMessage("");
          fetchMessages();
          setTimeout(() => {
            if (inputRef.current) {
              inputRef.current.focus();
            }
          }, 10);
        } else {
          console.error("Failed to send message:", r.message);
        }
      })
      .catch((e) => {
        console.error("Error sending message:", e);
      })
      .finally(() => {
        setSending(false);
        startFetchingMessages();
      });
  }

  async function askForDisconnect() {
    if (sending) {
      return;
    }
    setShowDisconnectDialog(true);
  }

  async function performDisconnect() {
    await stopFetchingMessages();
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
      <main className="flex flex-col grow-0 justify-between shrink-0 h-screen">
        <ScrollArea
          ref={scrollAreaRef}
          className="flex-grow w-full overflow-y-auto"
          scrollHideDelay={400}
          onScrollCapture={checkIfAtBottom}
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
      <Dialog
        open={showDisconnectDialog}
        onOpenChange={setShowDisconnectDialog}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Disconnect from server</DialogTitle>
            <DialogDescription>
              Are you sure you want to disconnect from the server?
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant={"outline"}>Cancel</Button>
            </DialogClose>
            <Button
              onClick={() => {
                setShowDisconnectDialog(false);
                performDisconnect();
              }}
            >
              Disconnect
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
