type MessageResponse = {
  message: Message | null;
  raw_string: string;
};

type Message = {
  client: string;
  content: string;
  username: string;
  timestamp: string;
};

export type { MessageResponse, Message };
