'use client';

import { useWeChatAssistant } from '@/hooks/useWeChatAssistant';

interface ContactSalesButtonProps {
  className?: string;
  children: React.ReactNode;
}

export default function ContactSalesButton({ className, children }: ContactSalesButtonProps) {
  const { openWeChatAssistant } = useWeChatAssistant();

  return (
    <button onClick={openWeChatAssistant} className={className}>
      {children}
    </button>
  );
}

