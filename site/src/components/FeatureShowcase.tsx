'use client';

import type { ReactNode } from 'react';
import Image from 'next/image';
import DownloadButton from '@/components/DownloadButton';
import { Locale } from '@/i18n/config';

type TranslateFn = (key: string) => string;

/** 大图 + SVG 动画（置顶） */
const SHOWCASE_KEYS = ['visualSelection', 'aiRecognition', 'recordActions', 'deepCollection'] as const;
type ShowcaseFeatureKey = (typeof SHOWCASE_KEYS)[number];

/** 底部 emoji 卡片：宽屏 3×2 */
const SIMPLE_CARD_FEATURES: { key: string; icon: string }[] = [
  { key: 'autoPagination', icon: '🔄' },
  { key: 'multipleFormats', icon: '📊' },
  { key: 'loginSupport', icon: '🔐' },
  { key: 'dynamicPages', icon: '🌐' },
  { key: 'scheduledTasks', icon: '📱' },
  { key: 'cloudSync', icon: '☁️' },
];

/** SVG 均在 `public/features/`，命名：`feature-<slug>.svg` */
const SHOWCASE_ASSETS: Record<ShowcaseFeatureKey, { src: string; width: number; height: number }> = {
  visualSelection: { src: '/features/feature-visual-selection.svg', width: 420, height: 260 },
  aiRecognition: { src: '/features/feature-ai-recognition.svg', width: 600, height: 400 },
  recordActions: { src: '/features/feature-record-actions.svg', width: 420, height: 260 },
  deepCollection: { src: '/features/feature-deep-collection.svg', width: 420, height: 260 },
};

/** AI 图为横版，略提高可视高度 */
const ILLUSTRATION_MAX_CLASS: Record<ShowcaseFeatureKey, string> = {
  visualSelection: 'max-h-[min(17rem,52vh)] sm:max-h-[min(19rem,52vh)] lg:max-h-[min(22rem,56vh)]',
  aiRecognition: 'max-h-[min(22rem,58vh)] sm:max-h-[min(24rem,58vh)] lg:max-h-[min(26rem,62vh)]',
  recordActions: 'max-h-[min(17rem,52vh)] sm:max-h-[min(19rem,52vh)] lg:max-h-[min(22rem,56vh)]',
  deepCollection: 'max-h-[min(17rem,52vh)] sm:max-h-[min(19rem,52vh)] lg:max-h-[min(22rem,56vh)]',
};

function Illustration({ featureKey }: { featureKey: ShowcaseFeatureKey }) {
  const { src, width, height } = SHOWCASE_ASSETS[featureKey];
  return (
    <Image
      src={src}
      alt=""
      width={width}
      height={height}
      className={`h-auto w-full object-contain ${ILLUSTRATION_MAX_CLASS[featureKey]}`}
      unoptimized
    />
  );
}

/** macOS 窗口标题栏 + 交通灯（装饰，无交互） */
function MacWindowChrome({ children }: { children: ReactNode }) {
  return (
    <div
      className="relative flex flex-col overflow-hidden rounded-xl bg-white shadow-[0_0_0_0.5px_rgba(0,0,0,0.06),0_1px_2px_rgba(0,0,0,0.04),0_4px_12px_rgba(0,0,0,0.06),0_20px_40px_-8px_rgba(0,0,0,0.1)]"
    >
      <div
        className="flex shrink-0 items-center gap-2 border-b border-black/[0.06] bg-[#ececec] px-3 py-2.5 sm:px-3.5"
        aria-hidden
      >
        <span className="h-3 w-3 rounded-full bg-[#ff5f57] shadow-[inset_0_-1px_0_rgba(0,0,0,0.15)] ring-1 ring-black/[0.12]" />
        <span className="h-3 w-3 rounded-full bg-[#febc2e] shadow-[inset_0_-1px_0_rgba(0,0,0,0.12)] ring-1 ring-black/[0.08]" />
        <span className="h-3 w-3 rounded-full bg-[#28c840] shadow-[inset_0_-1px_0_rgba(0,0,0,0.12)] ring-1 ring-black/[0.08]" />
      </div>
      <div className="bg-white px-2 pb-2 pt-2 sm:px-3 sm:pb-3 sm:pt-2.5">{children}</div>
    </div>
  );
}

function SimpleFeatureCard({ t, featureKey, icon }: { t: TranslateFn; featureKey: string; icon: string }) {
  return (
    <div className="bg-gradient-to-br from-slate-50 to-white p-6 rounded-xl border border-gray-200 hover:shadow-lg transition">
      <div className="text-4xl mb-4">{icon}</div>
      <h3 className="text-xl font-semibold text-gray-900 mb-2">{t(`features.${featureKey}.title`)}</h3>
      <p className="text-gray-600">{t(`features.${featureKey}.description`)}</p>
    </div>
  );
}

function ShowcaseBlock({
  t,
  lang,
  featureKey,
  showcaseIndex,
}: {
  t: TranslateFn;
  lang: Locale;
  featureKey: ShowcaseFeatureKey;
  showcaseIndex: number;
}) {
  const reverse = showcaseIndex % 2 === 1;
  return (
    <article>
      <div
        className={`flex flex-col gap-8 md:gap-10 lg:items-center lg:gap-12 xl:gap-14 ${
          reverse ? 'lg:flex-row-reverse' : 'lg:flex-row'
        }`}
      >
        {/* 主视觉：macOS 风格柔影 + 白底 */}
        <div className="relative min-w-0 lg:flex-[1.55]">
          <MacWindowChrome>
            <Illustration featureKey={featureKey} />
          </MacWindowChrome>
        </div>
        {/* 辅助说明：收窄、字级略小 */}
        <div className="min-w-0 lg:flex-1 lg:max-w-md xl:max-w-lg">
          <h3 className="text-lg font-semibold tracking-tight text-gray-800 sm:text-xl">{t(`features.${featureKey}.title`)}</h3>
          <p className="mt-3 text-sm leading-relaxed text-gray-500 sm:text-[15px]">{t(`features.${featureKey}.description`)}</p>
          <div className="mt-6">
            <DownloadButton lang={lang} variant="header" textKey="features.showcaseInstallCta" />
          </div>
        </div>
      </div>
    </article>
  );
}

function MoreFeaturesDivider({ t }: { t: TranslateFn }) {
  return (
    <div className="mx-auto max-w-6xl pt-6 md:pt-10" role="presentation">
      <div className="flex items-center gap-4 md:gap-6">
        <div className="h-px min-w-0 flex-1 bg-gradient-to-r from-transparent via-gray-200 to-gray-200" />
        <h2 className="shrink-0 text-center text-sm font-semibold text-gray-600 md:text-base">
          {t('features.moreSectionTitle')}
        </h2>
        <div className="h-px min-w-0 flex-1 bg-gradient-to-l from-transparent via-gray-200 to-gray-200" />
      </div>
    </div>
  );
}

export default function FeatureShowcase({ t, lang }: { t: TranslateFn; lang: Locale }) {
  return (
    <div className="space-y-16 md:space-y-24">
      {SHOWCASE_KEYS.map((key, index) => (
        <ShowcaseBlock key={key} t={t} lang={lang} featureKey={key} showcaseIndex={index} />
      ))}
      <MoreFeaturesDivider t={t} />
      <div className="mx-auto grid max-w-6xl grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 sm:gap-8">
        {SIMPLE_CARD_FEATURES.map((item) => (
          <SimpleFeatureCard key={item.key} t={t} featureKey={item.key} icon={item.icon} />
        ))}
      </div>
    </div>
  );
}
