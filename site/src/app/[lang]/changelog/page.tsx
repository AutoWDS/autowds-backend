import fs from 'fs';
import path from 'path';
import Link from 'next/link';
import { Locale } from '@/i18n/config';
import { useTranslations } from '@/i18n/utils';
import Header from '@/components/Header';
import Footer from '@/components/Footer';
import { parseChangelog, ChangelogEntry, getVersionBadgeColor, getChangeIcon } from '@/utils/changelog-parser';

export function generateStaticParams() {
  return [
    { lang: 'zh' },
    { lang: 'en' }
  ];
}

// Load changelog data from markdown files
function getChangelogData(lang: Locale): ChangelogEntry[] {
  const changelogPath = path.join(process.cwd(), lang === 'zh' ? 'CHANGELOG.zh.md' : 'CHANGELOG.md');
  
  try {
    const markdownContent = fs.readFileSync(changelogPath, 'utf-8');
    return parseChangelog(markdownContent);
  } catch (error) {
    console.error('Failed to load changelog:', error);
    return [];
  }
}

export default function Changelog({ params }: { params: { lang: Locale } }) {
  const t = useTranslations(params.lang);
  const changelogData = getChangelogData(params.lang);

  const getVersionTypeLabel = (type: ChangelogEntry['type']) => {
    const labels = {
      zh: {
        major: '重大更新',
        minor: '功能更新',
        patch: '修复更新',
        unreleased: '未发布'
      },
      en: {
        major: 'Major Release',
        minor: 'Feature Update',
        patch: 'Bug Fix',
        unreleased: 'Unreleased'
      }
    };
    return labels[params.lang][type];
  };

  const getChangeSectionLabel = (type: keyof ChangelogEntry['changes']) => {
    const labels = {
      zh: {
        added: '新增功能',
        changed: '功能改进',
        fixed: '问题修复',
        removed: '移除功能',
        deprecated: '已弃用',
        security: '安全修复'
      },
      en: {
        added: 'New Features',
        changed: 'Improvements',
        fixed: 'Bug Fixes',
        removed: 'Removed Features',
        deprecated: 'Deprecated',
        security: 'Security Fixes'
      }
    };
    return labels[params.lang][type];
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-50 to-white">
      <Header lang={params.lang} currentPage="changelog" />

      {/* Page Header */}
      <section className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 pt-16 pb-8">
        <div className="text-center">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">
            {t('changelog.title')}
          </h1>
          <p className="text-xl text-gray-600 mb-8">
            {t('changelog.subtitle')}
          </p>
        </div>
      </section>

      {/* Changelog Content */}
      <section className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 pb-20">
        <div className="space-y-12">
          {changelogData.map((entry, index) => (
            <div key={entry.version} className="bg-white rounded-xl shadow-md border border-gray-200 overflow-hidden">
              {/* Version Header */}
              <div className="bg-gradient-to-r from-slate-50 to-white px-6 py-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <h2 className="text-2xl font-bold text-gray-900">
                      v{entry.version}
                    </h2>
                    <span className={`px-3 py-1 rounded-full text-sm font-medium border ${getVersionBadgeColor(entry.type)}`}>
                      {getVersionTypeLabel(entry.type)}
                    </span>
                  </div>
                  <time className="text-gray-500 font-medium">
                    {entry.date ? new Date(entry.date).toLocaleDateString(params.lang === 'zh' ? 'zh-CN' : 'en-US') : ''}
                  </time>
                </div>
              </div>

              {/* Changes Content */}
              <div className="p-6">
                <div className="space-y-6">
                  {Object.entries(entry.changes).map(([changeType, changes]) => {
                    const typedChangeType = changeType as keyof ChangelogEntry['changes'];
                    return (
                      changes && changes.length > 0 && (
                        <div key={changeType}>
                          <h3 className="flex items-center text-lg font-semibold text-gray-900 mb-3">
                            <span className="mr-2">{getChangeIcon(typedChangeType)}</span>
                            {getChangeSectionLabel(typedChangeType)}
                          </h3>
                          <ul className="space-y-2">
                            {changes.map((change, changeIndex) => (
                              <li key={changeIndex} className="flex items-start">
                                <span className="w-2 h-2 bg-gray-400 rounded-full mt-2 mr-3 flex-shrink-0"></span>
                                <span className="text-gray-700">{change}</span>
                              </li>
                            ))}
                          </ul>
                        </div>
                      )
                    );
                  })}
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Statistics and Actions */}
        <div className="mt-12 space-y-8">
          {/* Version Statistics */}
          <div className="bg-gradient-to-r from-primary-50 to-blue-50 rounded-xl p-6">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
              <div>
                <div className="text-2xl font-bold text-primary-600">
                  {changelogData.length}
                </div>
                <div className="text-sm text-gray-600">
                  {params.lang === 'zh' ? '总版本数' : 'Total Versions'}
                </div>
              </div>
              <div>
                <div className="text-2xl font-bold text-blue-600">
                  {changelogData.filter(entry => entry.type === 'major').length}
                </div>
                <div className="text-sm text-gray-600">
                  {params.lang === 'zh' ? '重大更新' : 'Major Updates'}
                </div>
              </div>
              <div>
                <div className="text-2xl font-bold text-green-600">
                  {changelogData.filter(entry => entry.type === 'minor').length}
                </div>
                <div className="text-sm text-gray-600">
                  {params.lang === 'zh' ? '功能更新' : 'Feature Updates'}
                </div>
              </div>
              <div>
                <div className="text-2xl font-bold text-orange-600">
                  {changelogData.filter(entry => entry.type === 'patch').length}
                </div>
                <div className="text-sm text-gray-600">
                  {params.lang === 'zh' ? '修复更新' : 'Bug Fixes'}
                </div>
              </div>
            </div>
          </div>

          {/* Action Cards */}
          <div className="grid md:grid-cols-2 gap-6">
            {/* GitHub Releases */}
            <div className="bg-white rounded-xl shadow-md border border-gray-200 p-6">
              <div className="flex items-center mb-4">
                <div className="w-12 h-12 bg-gray-900 rounded-lg flex items-center justify-center mr-4">
                  <svg className="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                  </svg>
                </div>
                <div>
                  <h3 className="text-lg font-semibold text-gray-900">
                    {params.lang === 'zh' ? 'GitHub Releases' : 'GitHub Releases'}
                  </h3>
                  <p className="text-gray-600 text-sm">
                    {params.lang === 'zh' ? '查看完整的发布历史和下载' : 'View complete release history and downloads'}
                  </p>
                </div>
              </div>
              <a
                href="https://github.com/AutoWDS/autowds-backend/releases"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium transition"
              >
                {params.lang === 'zh' ? '访问 GitHub →' : 'Visit GitHub →'}
              </a>
            </div>

            {/* Learning Resources */}
            <div className="bg-white rounded-xl shadow-md border border-gray-200 p-6">
              <div className="flex items-center mb-4">
                <div className="w-12 h-12 bg-primary-600 rounded-lg flex items-center justify-center mr-4">
                  <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                  </svg>
                </div>
                <div>
                  <h3 className="text-lg font-semibold text-gray-900">
                    {params.lang === 'zh' ? '学习资源' : 'Learning Resources'}
                  </h3>
                  <p className="text-gray-600 text-sm">
                    {params.lang === 'zh' ? '了解如何使用新功能和最佳实践' : 'Learn how to use new features and best practices'}
                  </p>
                </div>
              </div>
              <Link
                href={`/${params.lang}/learn`}
                className="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium transition"
              >
                {params.lang === 'zh' ? '查看教程 →' : 'View Tutorials →'}
              </Link>
            </div>
          </div>
        </div>
      </section>

      <Footer lang={params.lang} />
    </div>
  );
}