import Link from 'next/link';
import Image from 'next/image';
import { Locale } from '@/i18n/config';
import { useTranslations } from '@/i18n/utils';
import ContactSalesLink from '@/components/ContactSalesLink';

interface FooterProps {
  lang: Locale;
  variant?: 'full' | 'simple';
}

export default function Footer({ lang, variant = 'full' }: FooterProps) {
  const t = useTranslations(lang);

  if (variant === 'simple') {
    return (
      <footer className="bg-gray-900 text-gray-400 py-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <p className="text-sm">{t('footer.copyright')}</p>
        </div>
      </footer>
    );
  }

  return (
    <footer className="bg-gray-900 text-gray-400 py-12">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid md:grid-cols-4 gap-8">
          <div>
            <div className="flex items-center space-x-2 mb-4">
              <Image src="/favicon.ico" alt="AutoWDS" width={32} height={32} />
              <span className="text-white font-bold">AutoWDS</span>
            </div>
            <p className="text-sm">{t('footer.description')}</p>
          </div>
          <div>
            <h4 className="text-white font-semibold mb-4">{t('footer.product')}</h4>
            <ul className="space-y-2 text-sm">
              <li><Link href={`/${lang}#features`} className="hover:text-white transition">{t('nav.features')}</Link></li>
              <li><Link href={`/${lang}#use-cases`} className="hover:text-white transition">{t('nav.useCases')}</Link></li>
              <li><Link href={`/${lang}#pricing`} className="hover:text-white transition">{t('nav.pricing')}</Link></li>
            </ul>
          </div>
          <div>
            <h4 className="text-white font-semibold mb-4">{t('footer.resources')}</h4>
            <ul className="space-y-2 text-sm">
              <li><Link href={`/${lang}/learn`} className="hover:text-white transition">{t('nav.tutorial')}</Link></li>
              <li><Link href={`/${lang}/changelog`} className="hover:text-white transition">{t('nav.changelog')}</Link></li>
              <li><Link href="/privacy" className="hover:text-white transition">Privacy Policy</Link></li>
            </ul>
          </div>
          <div>
            <h4 className="text-white font-semibold mb-4">{t('footer.contact')}</h4>
            <ul className="space-y-3 text-sm">
              <li>
                <a 
                  href="mailto:hff1996723@163.com" 
                  className="text-gray-400 hover:text-white transition flex items-center gap-2"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                  </svg>
                  <span>hff1996723@163.com</span>
                </a>
              </li>
              <li>
                <ContactSalesLink className="text-gray-400 hover:text-white transition flex items-center gap-2">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                  </svg>
                  <span>{t('common.wechatAssistant')}</span>
                </ContactSalesLink>
              </li>
            </ul>
          </div>
        </div>
        <div className="border-t border-gray-800 mt-8 pt-8 text-sm text-center">
          <p>{t('footer.copyright')}</p>
        </div>
      </div>
    </footer>
  );
}