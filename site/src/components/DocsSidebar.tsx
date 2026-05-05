'use client';

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Locale } from '@/i18n/config';
import { docsConfig } from '@/config/docs';

interface DocsSidebarProps {
  lang: Locale;
}

export default function DocsSidebar({ lang }: DocsSidebarProps) {
  const pathname = usePathname();
  const isZh = lang === 'zh';
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const sidebarContent = (
    <div className="p-6">
      <div className="mb-6">
        <h2 className="text-lg font-bold text-gray-900 mb-2">
          {isZh ? '文档' : 'Documentation'}
        </h2>
        <p className="text-sm text-gray-600">
          {isZh ? '学习如何使用 AutoWDS' : 'Learn how to use AutoWDS'}
        </p>
      </div>

      <nav className="space-y-6">
        {docsConfig.map((section, sectionIndex) => (
          <div key={sectionIndex}>
            <h3 className="text-sm font-semibold text-gray-900 mb-3">
              {section.title[lang]}
            </h3>
            <ul className="space-y-2">
              {section.items.map((item) => {
                const href = `/${lang}/docs/${item.slug}`;
                const isActive = pathname === href;
                
                return (
                  <li key={item.slug}>
                    <Link
                      href={href}
                      onClick={() => setIsMobileMenuOpen(false)}
                      className={`block px-3 py-2 rounded-lg text-sm transition ${
                        isActive
                          ? 'bg-primary-50 text-primary-700 font-medium'
                          : 'text-gray-700 hover:bg-gray-50 hover:text-gray-900'
                      }`}
                    >
                      {item.title[lang]}
                    </Link>
                  </li>
                );
              })}
            </ul>
          </div>
        ))}
      </nav>

      {/* 返回首页链接 */}
      <div className="mt-8 pt-6 border-t border-gray-200">
        <Link
          href={`/${lang}`}
          className="flex items-center text-sm text-gray-600 hover:text-gray-900 transition"
        >
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
          {isZh ? '返回首页' : 'Back to Home'}
        </Link>
      </div>
    </div>
  );

  return (
    <>
      {/* 移动端菜单按钮 */}
      <button
        onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
        className="lg:hidden fixed bottom-6 left-6 z-50 bg-primary-600 text-white p-4 rounded-full shadow-lg hover:bg-primary-700 transition"
        aria-label="Toggle menu"
      >
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          {isMobileMenuOpen ? (
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          ) : (
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
          )}
        </svg>
      </button>

      {/* 桌面端侧边栏 */}
      <aside className="hidden lg:block w-64 flex-shrink-0 bg-white border-r border-gray-200 overflow-y-auto sticky top-16 h-[calc(100vh-4rem)]">
        {sidebarContent}
      </aside>

      {/* 移动端侧边栏 */}
      {isMobileMenuOpen && (
        <>
          {/* 遮罩层 */}
          <div
            className="lg:hidden fixed inset-0 bg-black bg-opacity-50 z-40"
            onClick={() => setIsMobileMenuOpen(false)}
          />
          
          {/* 侧边栏 */}
          <aside className="lg:hidden fixed top-16 left-0 bottom-0 w-64 bg-white shadow-xl z-50 overflow-y-auto">
            {sidebarContent}
          </aside>
        </>
      )}
    </>
  );
}
