export interface Page<T> {
  content: T[];
  totalElements: number;
  size: number;
  number: number;
  /** summer `Page` 序列化字段（0 基页码），与 `number` 二选一 */
  page?: number;
  /** summer `Page` 序列化字段（snake_case 总条数） */
  total_elements?: number;
}

export interface DataCursorPage<T> {
  content: T[];
  total: number;
  size: number;
  offset: number;
  desc: boolean;
}
