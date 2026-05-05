import fs from 'fs';
import path from 'path';
import MarkdownRenderer from '@/components/MarkdownRenderer';

export default function PrivacyPage() {
  // Read the privacy policy markdown file
  const privacyPolicyPath = path.join(process.cwd(), 'policy', 'PRIVACY_POLICY.md');
  const content = fs.readFileSync(privacyPolicyPath, 'utf-8');

  return (
    <div className="min-h-screen bg-white">
      <main className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <article className="prose prose-lg max-w-none">
          <MarkdownRenderer content={content} />
        </article>
      </main>
    </div>
  );
}