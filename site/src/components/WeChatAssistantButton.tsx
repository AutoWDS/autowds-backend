'use client';

import { useWeChatAssistant } from '@/hooks/useWeChatAssistant';

export default function WeChatAssistantButton() {
  const { openWeChatAssistant, loading, error, t } = useWeChatAssistant();

  return (
    <div className="fixed bottom-6 right-6 z-50 flex flex-col items-end space-y-2">
      {error && (
        <div className="bg-red-50 text-red-700 text-xs px-3 py-2 rounded shadow">
          {error}
        </div>
      )}
      <button
        type="button"
        onClick={openWeChatAssistant}
        disabled={loading}
        className="flex items-center gap-2 bg-gradient-to-r from-primary-600 to-blue-500 text-white px-4 py-3 rounded-full shadow-lg hover:shadow-xl transition focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-70 disabled:cursor-not-allowed"
        aria-label={t('common.wechatAssistant')}
      >
        <span className="text-lg">💬</span>
        <span className="text-sm font-semibold">
          {loading ? t('common.wechatAssistantLoading') : t('common.wechatAssistant')}
        </span>
      </button>
    </div>
  );
}

