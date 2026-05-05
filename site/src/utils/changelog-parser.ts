export interface ChangelogEntry {
  version: string;
  date: string | null;
  type: 'major' | 'minor' | 'patch' | 'unreleased';
  changes: {
    added?: string[];
    changed?: string[];
    fixed?: string[];
    removed?: string[];
    deprecated?: string[];
    security?: string[];
  };
  compareUrl?: string;
}

export function parseChangelog(markdownContent: string): ChangelogEntry[] {
  const lines = markdownContent.split('\n');
  const entries: ChangelogEntry[] = [];
  let currentEntry: ChangelogEntry | null = null;
  let currentSection: keyof ChangelogEntry['changes'] | null = null;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();

    // Parse version headers (## [version] - date or ## [Unreleased])
    const versionMatch = line.match(/^##\s+\[([^\]]+)\](?:\s+-\s+(.+))?/);
    if (versionMatch) {
      // Save previous entry
      if (currentEntry) {
        entries.push(currentEntry);
      }

      const version = versionMatch[1];
      const date = versionMatch[2] || null;
      
      // Determine version type
      let type: ChangelogEntry['type'] = 'patch';
      if (version.toLowerCase() === 'unreleased' || version === '未发布') {
        type = 'unreleased';
      } else {
        const versionParts = version.split('.');
        if (versionParts.length >= 2) {
          const major = parseInt(versionParts[0]);
          const minor = parseInt(versionParts[1]);
          const patch = parseInt(versionParts[2] || '0');
          
          if (patch === 0 && minor === 0) {
            type = 'major';
          } else if (patch === 0) {
            type = 'minor';
          } else {
            type = 'patch';
          }
        }
      }

      currentEntry = {
        version,
        date,
        type,
        changes: {}
      };
      currentSection = null;
      continue;
    }

    // Parse section headers (### Added, ### Changed, etc.)
    const sectionMatch = line.match(/^###\s+(.+)/);
    if (sectionMatch && currentEntry) {
      const sectionName = sectionMatch[1].toLowerCase();
      
      // Map Chinese section names to English keys
      const sectionMapping: { [key: string]: keyof ChangelogEntry['changes'] } = {
        // English
        'added': 'added',
        'changed': 'changed',
        'fixed': 'fixed',
        'removed': 'removed',
        'deprecated': 'deprecated',
        'security': 'security',
        // Chinese
        '新增': 'added',
        '变更': 'changed',
        '修复': 'fixed',
        '移除': 'removed',
        '已弃用': 'deprecated',
        '安全': 'security'
      };
      
      const mappedSection = sectionMapping[sectionName];
      if (mappedSection) {
        currentSection = mappedSection;
        if (!currentEntry.changes[currentSection]) {
          currentEntry.changes[currentSection] = [];
        }
      }
      continue;
    }

    // Parse list items (- item)
    const listItemMatch = line.match(/^-\s+(.+)/);
    if (listItemMatch && currentEntry && currentSection) {
      const item = listItemMatch[1];
      if (!currentEntry.changes[currentSection]) {
        currentEntry.changes[currentSection] = [];
      }
      currentEntry.changes[currentSection]!.push(item);
      continue;
    }

    // Parse compare URLs at the end
    const compareMatch = line.match(/^\[([^\]]+)\]:\s+(.+)/);
    if (compareMatch) {
      const version = compareMatch[1];
      const url = compareMatch[2];
      
      // Find the entry with this version and add the compare URL
      const entry = entries.find(e => e.version === version);
      if (entry) {
        entry.compareUrl = url;
      } else if (currentEntry && currentEntry.version === version) {
        currentEntry.compareUrl = url;
      }
    }
  }

  // Add the last entry
  if (currentEntry) {
    entries.push(currentEntry);
  }

  return entries;
}

export function getVersionBadgeColor(type: ChangelogEntry['type']): string {
  switch (type) {
    case 'major':
      return 'bg-red-200 text-red-800 border-red-500';
    case 'minor':
      return 'bg-blue-200 text-blue-800 border-blue-500';
    case 'patch':
      return 'bg-green-200 text-green-800 border-green-500';
    case 'unreleased':
      return 'bg-purple-200 text-purple-800 border-purple-500';
    default:
      return 'bg-gray-200 text-gray-800 border-gray-500';
  }
}

export function getChangeIcon(type: keyof ChangelogEntry['changes']): string {
  switch (type) {
    case 'added':
      return '✨';
    case 'changed':
      return '🔄';
    case 'fixed':
      return '🐛';
    case 'removed':
      return '🗑️';
    case 'deprecated':
      return '⚠️';
    case 'security':
      return '🔒';
    default:
      return '📝';
  }
}