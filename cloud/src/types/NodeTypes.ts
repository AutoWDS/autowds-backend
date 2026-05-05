import _ from "lodash";

export enum NodeType {
  start = "start",
  page = "page",
  list = "list",
  detail = "detail",
}

export enum PaginationTypeDesc {
  scroll = "滚动翻页",
  click_next = "点击下一页",
  load_more = "加载更多",
}

export type PaginationType = keyof typeof PaginationTypeDesc;

export enum ExtractorTypeDesc {
  regex = "正则表达式",
  sed = "sed表达式",
  js = "js函数",
}

export type ExtractorType = keyof typeof ExtractorTypeDesc;

export interface Field {
  id: string;
  name: string;
  selector: string;
  attr: string;
  extractor: Extractor;
}

export interface Extractor {
  type?: ExtractorType;
  code?: string;
}

export interface Fields {
  fields: Field[];
}

export interface Steps {
  steps?: any[];
}

export interface PaginationConfig {
  selector: string;
}

export interface Pagination {
  type: PaginationType;
  config?: PaginationConfig;
}

export enum NewPageTypeDesc {
  click_element = "点击元素",
  open_url = "打开链接",
}

export type NewPageType = keyof typeof NewPageTypeDesc;

interface NodeConfig {}

export interface Viewport {
  width?: number;
  height?: number;
}

export interface HttpHeader {
  header: string;
  value: string;
}

// 页面节点，可以执行Steps
export interface StartNodeConfig extends NodeConfig, Steps {
  url: string;
  viewport: Viewport;
  httpHeaders: HttpHeader[];
}

export interface NewPageNodeConfig extends NodeConfig, Steps {
  type: NewPageType;
  value: string;
}

// 抽取节点，可以定义Fields抽取规则
export interface ListNodeConfig extends NodeConfig, Fields {
  listSelector: string;
  pagination?: Pagination;
}

export interface DetailNodeConfig extends NodeConfig, Fields {}

export interface Node {
  id: string;
  action: NodeType;
  config:
    | NodeConfig
    | StartNodeConfig
    | NewPageNodeConfig
    | ListNodeConfig
    | DetailNodeConfig;
}

export interface Edge {
  /** 边的 Source */
  source: string;
  /** 边的 Target */
  target: string;
}

export interface Graph {
  nodes: Node[];
  edges: Edge[];
}

export function getAllField({ nodes }: Graph): Field[] {
  return _.chain(nodes)
    .map((n) => (n.config as Fields).fields)
    .flatten()
    .compact()
    .value();
}
