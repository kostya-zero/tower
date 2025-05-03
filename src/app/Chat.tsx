"use client";

import MessageSection from "@/components/MessageSection";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Message } from "@/lib/types/message.types";
import {
  DoorOpenIcon,
  LoaderCircleIcon,
  SendHorizonalIcon,
} from "lucide-react";
import { useState } from "react";
import toast from "react-hot-toast";

export default function ChatPage() {
  const [sending, setSending] = useState(false);

  async function sendMessage() {
    setSending(true);
    await new Promise((resolve) => setTimeout(resolve, 3000));
    toast.error("Message failed to send");
    setSending(false);
  }

  const messages: Message[] = [
    {
      username: "Zero",
      client: "Tower",
      message:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
      timeStamp: new Date(),
    },
    {
      username: "Zero",
      client: "Tower",
      message:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
      
    },
  ];
  return (
    <main className="flex flex-col h-screen">
      <div className="h-full">
        {messages.map((message, index) => (
          <MessageSection message={message} key={index} />
        ))}
      </div>
      <div className="flex flex-row gap-2 p-2 border-t border-neutral-800">
        <Button size="icon" variant={"outline"} className="cursor-pointer">
          <DoorOpenIcon />
        </Button>
        <Input placeholder="Your message here..." disabled={sending} />
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
