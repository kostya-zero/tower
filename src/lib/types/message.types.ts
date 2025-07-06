type MessageResponse = {
  message: Message | null;
  rawString: string;
};

type Message = {
  client: string;
  content: string;
  username: string;
  timestamp?: string;
  avatar_url?: string;
};

export type { MessageResponse, Message };
