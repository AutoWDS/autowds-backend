import './globals.css';
import Script from 'next/script';
import WeChatAssistantButton from '@/components/WeChatAssistantButton';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html suppressHydrationWarning>
      <body suppressHydrationWarning>
        {children}
        <WeChatAssistantButton />
        {/* IconFont */}
        <Script src="//at.alicdn.com/t/c/font_5093665_p3o6zvy854.js" strategy="afterInteractive" />
        {/* Baidu Analytics */}
        <Script id="baidu-analytics" strategy="afterInteractive">
          {`
            var _hmt = _hmt || [];
            (function() {
              var hm = document.createElement("script");
              hm.src = "https://hm.baidu.com/hm.js?ded4b6222d11946afad44af5803878ba";
              var s = document.getElementsByTagName("script")[0]; 
              s.parentNode.insertBefore(hm, s);
            })();
          `}
        </Script>
      </body>
    </html>
  );
}
