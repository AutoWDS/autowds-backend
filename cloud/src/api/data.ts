import { DataStoreMeta, RDBField, StoreData, TableMetaType } from "types/Table";
import { DataCursorPage, Page } from "../types/Page";
import ajax from "../utils/ajax";

export interface TableFilter {
  name: string;
  type: string;
  createStart: string;
  createEnd: string;
  page: number;
  size: number;
  sort: string;
  desc: boolean;
}

const store = () => ajax("/store");

export async function queryStoreSchema(storeId: string) {
  return store().path(storeId).path("schema").get() as Promise<RDBField[]>;
}

export async function queryDataStore(filter: Partial<TableFilter>) {
  return store().query(filter).get() as Promise<Page<DataStoreMeta>>;
}

export async function saveDataStore(payload: {
  name: string;
  type: TableMetaType;
  fields?: RDBField[];
}) {
  return store().payload(payload).post() as Promise<DataStoreMeta>;
}

export async function deleteDataStore(storeId: string) {
  return store().path(storeId).delete();
}

export interface StoreDataQuery {
  offset?: number;
  desc?: boolean;
}

export async function queryStoreData(storeId: string, query: StoreDataQuery) {
  return store().path(storeId).path("data").query(query).get() as Promise<
    DataCursorPage<StoreData>
  >;
}
