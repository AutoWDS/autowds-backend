#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// 简单的解析器测试
function testParser() {
  const changelogZh = path.join(__dirname, '..', 'CHANGELOG.zh.md');
  const changelogEn = path.join(__dirname, '..', 'CHANGELOG.md');
  
  console.log('🧪 测试 Changelog 解析器\n');
  
  // 测试中文文件
  console.log('📄 测试中文 CHANGELOG.zh.md:');
  const zhContent = fs.readFileSync(changelogZh, 'utf-8');
  const zhLines = zhContent.split('\n');
  
  let versionCount = 0;
  let sectionCount = 0;
  let itemCount = 0;
  
  zhLines.forEach(line => {
    if (line.match(/^##\s+\[/)) versionCount++;
    if (line.match(/^###\s+/)) sectionCount++;
    if (line.match(/^-\s+/)) itemCount++;
  });
  
  console.log(`  ✓ 找到 ${versionCount} 个版本`);
  console.log(`  ✓ 找到 ${sectionCount} 个变更类型`);
  console.log(`  ✓ 找到 ${itemCount} 个变更项\n`);
  
  // 测试英文文件
  console.log('📄 测试英文 CHANGELOG.md:');
  const enContent = fs.readFileSync(changelogEn, 'utf-8');
  const enLines = enContent.split('\n');
  
  versionCount = 0;
  sectionCount = 0;
  itemCount = 0;
  
  enLines.forEach(line => {
    if (line.match(/^##\s+\[/)) versionCount++;
    if (line.match(/^###\s+/)) sectionCount++;
    if (line.match(/^-\s+/)) itemCount++;
  });
  
  console.log(`  ✓ 找到 ${versionCount} 个版本`);
  console.log(`  ✓ 找到 ${sectionCount} 个变更类型`);
  console.log(`  ✓ 找到 ${itemCount} 个变更项\n`);
  
  // 检查中文变更类型
  console.log('🔍 检查中文变更类型:');
  const zhSections = zhContent.match(/^###\s+(.+)$/gm) || [];
  const uniqueSections = [...new Set(zhSections.map(s => s.replace(/^###\s+/, '')))];
  console.log('  找到的变更类型:', uniqueSections.join(', '));
  
  const validZhSections = ['新增', '变更', '修复', '移除', '已弃用', '安全'];
  const invalidSections = uniqueSections.filter(s => !validZhSections.includes(s));
  
  if (invalidSections.length > 0) {
    console.log('  ⚠️  未识别的变更类型:', invalidSections.join(', '));
  } else {
    console.log('  ✓ 所有变更类型都有效\n');
  }
  
  console.log('✅ 解析器测试完成');
}

testParser();