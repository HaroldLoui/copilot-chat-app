declare interface ChatBox {
  id: number | string;
  title: string;
  count: number;
  createTime: string;
}

declare interface Message {
  id: string;
  sender: "AI" | "ME";
  content: string;
  createTime: string;
}
