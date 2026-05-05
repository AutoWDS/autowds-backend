'use client';

import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface MarkdownRendererProps {
  content: string;
}

export default function MarkdownRenderer({ content }: MarkdownRendererProps) {
  return (
    <ReactMarkdown 
      remarkPlugins={[remarkGfm]}
      components={{
        h1: ({node, ...props}) => <h1 className="text-4xl font-bold text-gray-900 mb-6" {...props} />,
        h2: ({node, ...props}) => <h2 className="text-3xl font-bold text-gray-900 mt-8 mb-4" {...props} />,
        h3: ({node, ...props}) => <h3 className="text-2xl font-semibold text-gray-900 mt-6 mb-3" {...props} />,
        p: ({node, ...props}) => <p className="text-gray-700 mb-4 leading-relaxed" {...props} />,
        ul: ({node, ...props}) => <ul className="list-disc list-inside mb-4 space-y-2" {...props} />,
        ol: ({node, ...props}) => <ol className="list-decimal list-inside mb-4 space-y-2" {...props} />,
        li: ({node, ...props}) => <li className="text-gray-700" {...props} />,
        a: ({node, ...props}) => <a className="text-primary-600 hover:underline" {...props} />,
        code: ({node, inline, ...props}: any) => 
          inline 
            ? <code className="bg-gray-100 text-primary-600 px-2 py-1 rounded text-sm" {...props} />
            : <code className="block bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto mb-4" {...props} />,
        blockquote: ({node, ...props}) => <blockquote className="border-l-4 border-primary-500 pl-4 italic text-gray-600 my-4" {...props} />,
      }}
    >
      {content}
    </ReactMarkdown>
  );
}
