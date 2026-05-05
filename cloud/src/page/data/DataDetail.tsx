import { queryStoreData } from "api/data";
import { useEffect, useRef, useState } from "react";

import { Empty, List, theme } from "antd";
import VirtualList from "rc-virtual-list";
import { useParams } from "react-router-dom";
import { DataCursorPage } from "types/Page";
import { StoreData } from "types/Table";

const DataDetail = () => {
  const {
    token: { colorBgContainer },
  } = theme.useToken();
  const { storeId } = useParams();
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState({} as DataCursorPage<StoreData>);
  const containerRef = useRef<HTMLDivElement | null>();

  const query = async (storeId: string, desc: boolean = false) => {
    setLoading(true);
    try {
      const data = await queryStoreData(storeId, { desc });
      setPage(data);
    } catch (e) {
      console.log(e);
    }
    setLoading(false);
  };

  useEffect(() => {
    storeId && query(storeId);
  }, [storeId]);

  const loadMoreData = async () => {
    if (loading) {
      return;
    }
    setLoading(true);
    try {
      const data = await queryStoreData(storeId as string, {
        offset: page.offset,
        desc: page.desc,
      });
      setPage((oldPage) => {
        const newPage = data;
        newPage.content = oldPage.content.concat(page.content);
        return newPage;
      });
    } catch (e) {
      console.log(e);
    }
    setLoading(false);
  };

  const containerHeight = containerRef.current?.clientHeight;

  return (
    <div
      ref={(ref) => (containerRef.current = ref)}
      style={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        background: colorBgContainer,
        height: "calc(100% - 16px)",
      }}
    >
      {page.content?.length ? (
        <List style={{ width: "100%", height: "100%" }}>
          <VirtualList
            data={page.content}
            height={containerHeight}
            itemHeight={50}
            itemKey="email"
            onScroll={(e) => {
              if (
                e.currentTarget.scrollHeight - e.currentTarget.scrollTop ===
                containerHeight
              )
                loadMoreData();
            }}
          >
            {(item: StoreData) => (
              <List.Item key={item.id}>{JSON.stringify(item.data)}</List.Item>
            )}
          </VirtualList>
        </List>
      ) : (
        <Empty />
      )}
    </div>
  );
};

export default DataDetail;
