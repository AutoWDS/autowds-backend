/**
 * i18n
 */
export enum TemplateTopicDesc {
  SocialNetwork = "enum_SOCIAL_NETWORK",
  ResearchEducation = "enum_RESEARCH_EDUCATION",
  ECommerce = "enum_E_COMMERCE",
  LocalLife = "enum_LOCAL_LIFE",
  Bidding = "enum_BIDDING",
  Media = "enum_MEDIA",
  SearchEngine = "enum_SEARCH_ENGINE",
  Other = "enum_OTHER",
}

export enum ProductEditionDesc {
  L0 = "enum_L0",
  L1 = "enum_L1",
  L2 = "enum_L2",
  L3 = "enum_L3",
}

export enum ProductEditionColor {
  L0 = "green",
  L1 = "cyan",
  L2 = "pink",
  L3 = "red",
}

export type TemplateTopic = keyof typeof TemplateTopicDesc;
export type ProductEdition = keyof typeof ProductEditionDesc;

export interface Template {
  id: string;
  name: string;
  detail: string;
  img: string;
  lang: string;
  topic: TemplateTopic;
  edition: ProductEdition;
  favCount: number;
  rule: any;
  created: string;
  modified: string;
  like: boolean;
}

export interface Favorite {
  userId: string;
  templateId: string;
}
