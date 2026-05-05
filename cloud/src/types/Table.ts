export enum TableMetaTypeDesc {
  DOC = "文档型",
  RDB = "关系型",
}

export type TableMetaType = keyof typeof TableMetaTypeDesc;

export interface DataStoreMeta {
  id: string;
  userId: string;
  name: string;
  created: number;
  count: number;
  bytes: number;
  type: TableMetaType;
}

export interface RDBField {
  name: string;
  defaultValue: string;
}

export interface StoreData {
  id: string;
  created: string;
  modified: string;
  data: any;
}
