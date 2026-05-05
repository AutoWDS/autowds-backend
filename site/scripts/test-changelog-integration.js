#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// 模拟解析器逻辑来测试
function parseChangelog(markdownContent) {
  const lines = markdownContent.split('\n');
  const entries = [];
  let currentEntry = null;
  let currentSection = null;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();

    // Parse version headers
    const versionMatch = line.match(/^##\s+\[([^\]]+)\](?:\s+-\s+(.+))?/);
    if (versionMatch) {
      if (currentEntry) {
        entries.push(currentEntry);
      }

      const version = versionMatch[1];
      const date = versionMatch[2] || null;
      
      let type = 'patch';
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

    // Parse section headers
    const sectionMatch = line.match(/^###\s+(.+)/);
    if (sectionMatch && currentEntry) {
      const sectionName = sectionMatch[1].toLowerCase();
      
      const sectionMapping = {
        'added': 'added',
        'changed': 'changed',
        'fixed': 'fixed',
        'removed': 'removed',
        'deprecated': 'deprecated',
        'security': 'security',
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

    // Parse list items
    const listItemMatch = line.match(/^-\s+(.+)/);
    if (listItemMatch && currentEntry && currentSection) {
      const item = listItemMatch[1];
      if (!currentEntry.changes[currentSection]) {
        currentEntry.changes[currentSection] = [];
      }
      currentEntry.changes[currentSection].push(item);
    }
  }

  if (currentEntry) {
    entries.push(currentEntry);
  }

  return entries;
}

// 测试中文 changelog
console.log('🧪 测试中文 Changelog 解析集成\n');

const changelogZh = path.join(__dirname, '..', 'CHANGELOG.zh.md');
const zhContent = fs.readFileSync(changelogZh, 'utf-8');
const zhEntries = parseChangelog(zhContent);

console.log(`✅ 成功解析 ${zhEntries.length} 个版本条目\n`);

zhEntries.slice(0, 3).forEach((entry, index) => {
  console.log(`📦 版本 ${entry.version} (${entry.type})`);
  console.log(`   日期: ${entry.date || '未指定'}`);
  
  Object.entries(entry.changes).forEach(([type, changes]) => {
    if (changes && changes.length > 0) {
      console.log(`   ${type}: ${changes.length} 项`);
    }
  });
  console.log('');
});

// 检查是否有空的条目
const emptyEntries = zhEntries.filter(entry => 
  Object.values(entry.changes).every(changes => !changes || changes.length === 0)
);

if (emptyEntries.length > 0) {
  console.log(`⚠️  发现 ${emptyEntries.length} 个空的版本条目:`);
  emptyEntries.forEach(entry => {
    console.log(`   - ${entry.version}`);
  });
} else {
  console.log('✅ 所有版本条目都包含变更内容');
}

console.log('\n🎉 中文 Changelog 解析测试完成！');