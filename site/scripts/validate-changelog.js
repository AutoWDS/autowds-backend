#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * 验证 changelog 文件格式是否符合 Keep a Changelog 规范
 */
function validateChangelog(filePath) {
  console.log(`验证 ${filePath}...`);
  
  if (!fs.existsSync(filePath)) {
    console.error(`❌ 文件不存在: ${filePath}`);
    return false;
  }

  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');
  
  let isValid = true;
  let hasTitle = false;
  let hasDescription = false;
  let versionCount = 0;
  
  // 检查基本结构
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    
    // 检查标题
    if (line.match(/^#\s+/)) {
      hasTitle = true;
    }
    
    // 检查描述
    if (line.includes('Keep a Changelog') || line.includes('keepachangelog.com')) {
      hasDescription = true;
    }
    
    // 检查版本格式
    const versionMatch = line.match(/^##\s+\[([^\]]+)\](?:\s+-\s+(.+))?/);
    if (versionMatch) {
      versionCount++;
      const version = versionMatch[1];
      const date = versionMatch[2];
      
      // 验证版本格式
      if (version !== 'Unreleased' && version !== '未发布') {
        const semverPattern = /^\d+\.\d+\.\d+$/;
        if (!semverPattern.test(version)) {
          console.warn(`⚠️  版本号格式可能不正确: ${version} (行 ${i + 1})`);
        }
      }
      
      // 验证日期格式
      if (date && version !== 'Unreleased' && version !== '未发布') {
        const datePattern = /^\d{4}-\d{2}-\d{2}$/;
        if (!datePattern.test(date)) {
          console.warn(`⚠️  日期格式可能不正确: ${date} (行 ${i + 1})`);
        }
      }
    }
    
    // 检查变更类型
    const sectionMatch = line.match(/^###\s+(.+)/);
    if (sectionMatch) {
      const section = sectionMatch[1];
      const validSections = [
        'Added', 'Changed', 'Deprecated', 'Removed', 'Fixed', 'Security',
        '新增', '变更', '已弃用', '移除', '修复', '安全'
      ];
      
      if (!validSections.includes(section)) {
        console.warn(`⚠️  未知的变更类型: ${section} (行 ${i + 1})`);
      }
    }
  }
  
  // 验证结果
  if (!hasTitle) {
    console.error('❌ 缺少主标题');
    isValid = false;
  }
  
  if (!hasDescription) {
    console.warn('⚠️  建议添加 Keep a Changelog 的引用说明');
  }
  
  if (versionCount === 0) {
    console.error('❌ 没有找到任何版本记录');
    isValid = false;
  }
  
  if (isValid) {
    console.log(`✅ ${path.basename(filePath)} 格式验证通过 (找到 ${versionCount} 个版本)`);
  }
  
  return isValid;
}

// 验证两个 changelog 文件
const changelogEn = path.join(__dirname, '..', 'CHANGELOG.md');
const changelogZh = path.join(__dirname, '..', 'CHANGELOG.zh.md');

console.log('🔍 验证 Changelog 文件格式...\n');

const enValid = validateChangelog(changelogEn);
console.log('');
const zhValid = validateChangelog(changelogZh);

console.log('\n📊 验证结果:');
if (enValid && zhValid) {
  console.log('✅ 所有 changelog 文件格式正确');
  process.exit(0);
} else {
  console.log('❌ 发现格式问题，请检查上述警告和错误');
  process.exit(1);
}