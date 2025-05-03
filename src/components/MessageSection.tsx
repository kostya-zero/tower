import { type Message } from "@/lib/types/message.types";
import { format } from "date-fns";

type Props = {
  message: Message;
};

export default function MessageSection({ message }: Props) {
  return (
    <figure className="flex flex-col p-4 gap-2 border-b border-neutral-800">
      <div className="flex flex-row justify-between items-center">
        <div className="flex flex-row font-bold gap-2 items-center">
          {message.username}
          <span className="text-neutral-500"> via {message.client}</span>
        </div>
        <p className="text-neutral-500">
          {message.timeStamp
            ? format(new Date(message.timeStamp), "MMM d, yyyy h:mm a")
            : "In this timeline"}
        </p>
      </div>
      <p className="text-neutral-300">{message.message}</p>
    </figure>
  );
}
