import { Locale, locales } from '@/i18n/config';

export async function generateStaticParams() {
  return locales.map((lang) => ({ lang }));
}

export async function generateMetadata({ params }: { params: { lang: Locale } }) {
  return {
    title: params.lang === 'zh' 
      ? 'AutoWDS - 强大的浏览器自动化插件'
      : 'AutoWDS - Powerful Browser Automation Extension',
    description: params.lang === 'zh'
      ? '一款功能强大的浏览器扩展，让网页数据采集和自动化操作变得简单高效'
      : 'A powerful browser extension that makes web scraping and automation simple and efficient',
  };
}

export default function LangLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return children;
}
