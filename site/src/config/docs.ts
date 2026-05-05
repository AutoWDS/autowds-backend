// 文档配置和导航结构
export interface DocItem {
  slug: string;
  title: {
    zh: string;
    en: string;
  };
}

export interface DocSection {
  title: {
    zh: string;
    en: string;
  };
  items: DocItem[];
}

export const docsConfig: DocSection[] = [
  {
    title: {
      zh: '快速开始',
      en: 'Getting Started'
    },
    items: [
      {
        slug: 'introduction',
        title: {
          zh: '产品介绍',
          en: 'Introduction'
        }
      },
      {
        slug: 'installation',
        title: {
          zh: '安装指南',
          en: 'Installation'
        }
      },
      {
        slug: 'getting-started',
        title: {
          zh: '快速入门',
          en: 'Getting Started'
        }
      }
    ]
  },
  {
    title: {
      zh: '核心概念',
      en: 'Core Concepts'
    },
    items: [
      {
        slug: 'basic-concepts',
        title: {
          zh: '基础概念',
          en: 'Basic Concepts'
        }
      },
      {
        slug: 'selectors',
        title: {
          zh: '选择器',
          en: 'Selectors'
        }
      }
    ]
  },
  {
    title: {
      zh: '功能指南',
      en: 'Feature Guides'
    },
    items: [
      {
        slug: 'visual-rule-development',
        title: {
          zh: '可视化规则开发',
          en: 'Visual Rule Development'
        }
      },
      {
        slug: 'intelligent-collection',
        title: {
          zh: '智能采集',
          en: 'Intelligent Collection'
        }
      },
      {
        slug: 'pagination',
        title: {
          zh: '分页处理',
          en: 'Pagination'
        }
      },
      {
        slug: 'scheduled-tasks',
        title: {
          zh: '定时任务',
          en: 'Scheduled Tasks'
        }
      },
      {
        slug: 'data-export',
        title: {
          zh: '数据导出',
          en: 'Data Export'
        }
      }
    ]
  },
  {
    title: {
      zh: '教程与示例',
      en: 'Tutorials & Examples'
    },
    items: [
      {
        slug: 'tutorials',
        title: {
          zh: '实战教程',
          en: 'Tutorials'
        }
      }
    ]
  },
  {
    title: {
      zh: '帮助与支持',
      en: 'Help & Support'
    },
    items: [
      {
        slug: 'faq',
        title: {
          zh: '常见问题',
          en: 'FAQ'
        }
      }
    ]
  }
];
