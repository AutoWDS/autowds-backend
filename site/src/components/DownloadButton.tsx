'use client';

import { useTranslations } from '@/i18n/utils';
import { Locale } from '@/i18n/config';

interface DownloadButtonProps {
  lang: Locale;
  variant?: 'hero' | 'header' | 'cta';
  textKey?: string;
}

export default function DownloadButton({ lang, variant = 'hero', textKey }: DownloadButtonProps) {
  const t = useTranslations(lang);
  
  // 检测浏览器类型并返回对应的下载链接
  const detectBrowserAndGetUrl = () => {
    if (typeof window === 'undefined') {
      // 服务端渲染时返回默认Chrome链接
      return 'https://chromewebstore.google.com/detail/autowds/edokboookdopdndgnhiihmimlljefldp';
    }

    const userAgent = window.navigator.userAgent;
    
    // 检测Firefox - 更精确的匹配
    if (/Firefox\/[\d.]+/.test(userAgent)) {
      return 'https://addons.mozilla.org/firefox/addon/octopus-crawler/';
    }
    
    // 检测Edge - 新版Edge使用Edg，老版Edge使用Edge
    if (/Edg\/[\d.]+/.test(userAgent) || /Edge\/[\d.]+/.test(userAgent)) {
      return 'https://microsoftedge.microsoft.com/addons/detail/autowds/nbckdijndlleiongachkolhipiigifec';
    }
    
    // 检测Chrome - 确保是真正的Chrome而不是其他Chromium浏览器
    if (/Chrome\/[\d.]+/.test(userAgent) && !/Edg\//.test(userAgent) && !/OPR\//.test(userAgent)) {
      return 'https://chromewebstore.google.com/detail/autowds/edokboookdopdndgnhiihmimlljefldp';
    }
    
    // 默认返回Chrome链接 (兜底方案，包括Safari、Opera等其他浏览器)
    return 'https://chromewebstore.google.com/detail/autowds/edokboookdopdndgnhiihmimlljefldp';
  };

  const handleDownload = () => {
    const downloadUrl = detectBrowserAndGetUrl();
    window.open(downloadUrl, '_blank', 'noopener,noreferrer');
  };
  
  // 根据变体确定按钮文本和样式
  const getButtonText = () => {
    if (textKey) return t(textKey);
    switch (variant) {
      case 'header':
        return t('common.installNow');
      case 'cta':
        return t('cta.button');
      default:
        return t('hero.downloadFree');
    }
  };

  const getButtonClasses = () => {
    const baseClasses = 'rounded-lg font-semibold transition flex items-center justify-center space-x-2';
    switch (variant) {
      case 'header':
        return `${baseClasses} bg-gradient-to-r from-primary-600 to-blue-500 text-white px-5 py-2 hover:shadow-lg text-sm md:text-base whitespace-nowrap`;
      case 'cta':
        return `${baseClasses} bg-white text-primary-600 px-8 py-4 hover:shadow-2xl transform hover:-translate-y-1 text-lg`;
      default:
        return `${baseClasses} bg-gradient-to-r from-primary-600 to-blue-500 text-white px-8 py-4 hover:shadow-xl transform hover:-translate-y-0.5 text-lg w-full sm:w-auto`;
    }
  };

  return (
    <button
      onClick={handleDownload}
      className={getButtonClasses()}
    >
      <span>{getButtonText()}</span>
      <svg
        className="w-5 h-5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
    </button>
  );
}