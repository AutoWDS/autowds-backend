'use client';

import { useEffect } from 'react';
import Image from 'next/image';

export default function RootPage() {
  useEffect(() => {
    const browserLang = navigator.language.toLowerCase();
    const locale = browserLang.startsWith('zh') ? 'zh' : 'en';
    window.location.replace('/' + locale + '/');
  }, []);

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-b from-slate-50 to-white">
      <div className="text-center">
        <div className="mx-auto mb-4 animate-pulse">
          <Image src="/favicon.ico" alt="AutoWDS" width={64} height={64} />
        </div>
        <p className="text-gray-600">Loading...</p>
      </div>
    </div>
  );
}
