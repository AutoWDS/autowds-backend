'use client';

import { useWeChatAssistant } from '@/hooks/useWeChatAssistant';

interface ContactSalesLinkProps {
  className?: string;
  children: React.ReactNode;
}

export default function ContactSalesLink({ className, children }: ContactSalesLinkProps) {
  const { openWeChatAssistant } = useWeChatAssistant();

  const handleClick = (e: React.MouseEvent<HTMLAnchorElement>) => {
    e.preventDefault();
    openWeChatAssistant();
  };

  return (
    <a href="#" onClick={handleClick} className={className}>
      {children}
    </a>
  );
}

