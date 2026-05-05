'use client';

import Link from 'next/link';
import Image from 'next/image';
import { useState } from 'react';
import { Locale } from '@/i18n/config';
import { useTranslations } from '@/i18n/utils';
import LanguageSwitcher from '@/components/LanguageSwitcher';
import DownloadButton from '@/components/DownloadButton';

interface HeaderProps {
  lang: Locale;
  currentPage?: 'home' | 'learn' | 'changelog' | 'docs';
}

export default function Header({ lang, currentPage = 'home' }: HeaderProps) {
  const t = useTranslations(lang);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const getLinkClassName = (page: string) => {
    const baseClass = "text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm md:text-base";
    const activeClass = "text-primary-600 font-medium transition whitespace-nowrap text-sm md:text-base";
    return currentPage === page ? activeClass : baseClass;
  };

  const getMobileLinkClassName = (page: string) => {
    const baseClass = "block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition";
    const activeClass = "block px-3 py-2 text-base font-medium text-primary-600 bg-primary-50 rounded-md transition";
    return currentPage === page ? activeClass : baseClass;
  };

  const toggleMobileMenu = () => {
    setIsMobileMenuOpen(!isMobileMenuOpen);
  };

  return (
    <header className="border-b bg-white/80 backdrop-blur-sm sticky top-0 z-50">
      <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="h-16 flex items-center justify-between">
          <Link href={`/${lang}`} className="flex items-center space-x-2">
            <Image src="/favicon.ico" alt="AutoWDS" width={32} height={32} />
            <span className="text-xl font-bold bg-gradient-to-r from-primary-600 to-blue-500 bg-clip-text text-transparent">
              AutoWDS
            </span>
          </Link>
          
          {/* Desktop Menu */}
          <div className="hidden lg:flex items-center space-x-6 xl:space-x-8">
            {currentPage === 'home' ? (
              <>
                <a
                  href="#features"
                  className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
                >
                  {t('nav.features')}
                </a>
                <a
                  href="#use-cases"
                  className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
                >
                  {t('nav.useCases')}
                </a>
                <a
                  href="#pricing"
                  className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
                >
                  {t('nav.pricing')}
                </a>
              </>
            ) : (
              <>
                <Link
                  href={`/${lang}#features`}
                  className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
                >
                  {t('nav.features')}
                </Link>
                <Link
                  href={`/${lang}#use-cases`}
                  className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
                >
                  {t('nav.useCases')}
                </Link>
                <Link
                  href={`/${lang}#pricing`}
                  className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
                >
                  {t('nav.pricing')}
                </Link>
              </>
            )}
            <Link
              href={`/${lang}/learn`}
              className={getLinkClassName('learn').replace('md:text-base', 'xl:text-base')}
            >
              {t('nav.tutorial')}
            </Link>
            <Link
              href={`/${lang}/changelog`}
              className={getLinkClassName('changelog').replace('md:text-base', 'xl:text-base')}
            >
              {t('nav.changelog')}
            </Link>
            <a
              href="https://autowds.dtiku.cn/cloud/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-600 hover:text-gray-900 transition whitespace-nowrap text-sm xl:text-base"
            >
              {t('nav.cloudService')}
            </a>
            <LanguageSwitcher currentLocale={lang} />
            <DownloadButton lang={lang} variant="header" />
          </div>

          {/* Mobile menu button */}
          <div className="lg:hidden flex items-center space-x-2">
            <LanguageSwitcher currentLocale={lang} />
            <button
              onClick={toggleMobileMenu}
              className="inline-flex items-center justify-center p-2 rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-primary-500"
              aria-expanded="false"
            >
              <span className="sr-only">Open main menu</span>
              {!isMobileMenuOpen ? (
                <svg className="block h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
                </svg>
              ) : (
                <svg className="block h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              )}
            </button>
          </div>
        </div>

        {/* Mobile Menu */}
        {isMobileMenuOpen && (
          <div className="lg:hidden">
            <div className="px-2 pt-2 pb-3 space-y-1 bg-white border-t">
              {currentPage === 'home' ? (
                <>
                  <a
                    href="#features"
                    className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {t('nav.features')}
                  </a>
                  <a
                    href="#use-cases"
                    className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {t('nav.useCases')}
                  </a>
                  <a
                    href="#pricing"
                    className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {t('nav.pricing')}
                  </a>
                </>
              ) : (
                <>
                  <Link
                    href={`/${lang}#features`}
                    className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {t('nav.features')}
                  </Link>
                  <Link
                    href={`/${lang}#use-cases`}
                    className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {t('nav.useCases')}
                  </Link>
                  <Link
                    href={`/${lang}#pricing`}
                    className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {t('nav.pricing')}
                  </Link>
                </>
              )}
              <Link
                href={`/${lang}/learn`}
                className={getMobileLinkClassName('learn')}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                {t('nav.tutorial')}
              </Link>
              <Link
                href={`/${lang}/changelog`}
                className={getMobileLinkClassName('changelog')}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                {t('nav.changelog')}
              </Link>
              <a
                href="https://autowds.dtiku.cn/cloud/"
                target="_blank"
                rel="noopener noreferrer"
                className="block px-3 py-2 text-base font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded-md transition"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                {t('nav.cloudService')}
              </a>
            </div>
          </div>
        )}
      </nav>
    </header>
  );
}