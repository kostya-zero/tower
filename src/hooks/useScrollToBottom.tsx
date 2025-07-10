import { useRef, useState } from "react";

export function useScrollToBottom() {
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const [isAtBottom, setIsAtBottom] = useState(true);

  const getScrollContainer = () => {
    return scrollAreaRef.current?.querySelector(
      "[data-radix-scroll-area-viewport]",
    ) as HTMLElement | null;
  };

  const checkIfAtBottom = () => {
    const scrollContainer = getScrollContainer();
    if (!scrollContainer) return;

    const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
    const threshold = 10;
    const atBottom = scrollTop + clientHeight >= scrollHeight - threshold;
    setIsAtBottom(atBottom);
  };

  const scrollToBottom = () => {
    const scrollContainer = getScrollContainer();
    if (scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  };

  return {
    scrollAreaRef,
    isAtBottom,
    setIsAtBottom,
    checkIfAtBottom,
    scrollToBottom,
  };
}
