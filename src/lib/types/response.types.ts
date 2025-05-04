import { Message } from "./message.types";

export type Response = {
  message?: Message;
  error?: boolean;
  rawMessage?: string;
};
