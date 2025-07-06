import { type Message } from "@/lib/types/message.types";

type Props = {
  message: Message;
};

export default function MessageSection({ message }: Props) {
  return (
    <figure className="flex w-full flex-row items-start gap-3 border-b border-neutral-800 p-4">
      {message.avatar_url ? (
        <img
          src={message.avatar_url}
          alt={`Avatar of ${message.username}`}
          className={
            "size-10 flex-shrink-0 flex-grow-0 rounded-sm bg-neutral-800"
          }
        />
      ) : (
        <div
          className={
            "size-10 flex-shrink-0 flex-grow-0 rounded-sm bg-neutral-800"
          }
        ></div>
      )}
      <div className="flex max-w-full flex-col select-none">
        <div className="flex flex-row items-center gap-2 leading-tight">
          <p className="font-semibold">{message.username}</p>
          <span className="font-normal text-neutral-500">
            via {message.client}
          </span>
          <p className="text-sm font-light text-neutral-500">
            ({message.timestamp ? message.timestamp : "In this timeline"})
          </p>
        </div>
        <p className="overflow-hidden text-sm break-words break-all text-neutral-300 select-text">
          {message.content.trim()}
        </p>
      </div>
    </figure>
  );
}
