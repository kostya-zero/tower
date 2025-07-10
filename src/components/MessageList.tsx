import { ScrollArea } from "@/components/ui/scroll-area";
import { MessageResponse } from "@/lib/types/message.types";
import BadMessageSection from "@/components/BadMessageSection";
import MessageSection from "@/components/MessageSection";
import { ForwardedRef, forwardRef } from "react";

interface MessageListProps {
  messages: MessageResponse[];
  onScroll: () => void;
}

const MessageList = forwardRef<HTMLDivElement, MessageListProps>(
  ({ messages, onScroll }, ref: ForwardedRef<HTMLDivElement>) => {
    return (
      <ScrollArea
        ref={ref}
        className="w-full flex-grow overflow-y-auto"
        scrollHideDelay={400}
        onScrollCapture={onScroll}
      >
        {messages.map((message, index) =>
          message.message ? (
            <MessageSection message={message.message} key={index} />
          ) : (
            <BadMessageSection rawString={message.rawString} key={index} />
          ),
        )}
      </ScrollArea>
    );
  },
);

MessageList.displayName = "MessageList";

export default MessageList;
