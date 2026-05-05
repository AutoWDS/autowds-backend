'use client';

import { ReactNode } from 'react';

interface ScrollToButtonProps {
  children: ReactNode;
  targetId: string;
  className?: string;
  scrollOptions?: ScrollIntoViewOptions;
}

export default function ScrollToButton({ 
  children, 
  targetId, 
  className,
  scrollOptions = { behavior: 'smooth', block: 'start' }
}: ScrollToButtonProps) {
  const handleClick = () => {
    const targetElement = document.getElementById(targetId);
    if (targetElement) {
      targetElement.scrollIntoView(scrollOptions);
    }
  };

  return (
    <button onClick={handleClick} className={className}>
      {children}
    </button>
  );
}