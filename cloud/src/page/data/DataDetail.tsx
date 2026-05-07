import { queryStoreData } from "api/data";
import { useEffect, useRef, useState } from "react";

import { BranchesOutlined } from "@ant-design/icons";
import { Button, Empty, List, Space, theme } from "antd";
import VirtualList from "rc-virtual-list";
import { useNavigate, useParams } from "react-router-dom";
import { DataCursorPage } from "types/Page";
import { StoreData } from "types/Table";

const DataDetail = () => {
  const {
    token: { colorBgContainer },
  } = theme.useToken();
  const { storeId } = useParams();
  const navigate = useNavigate();
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
      style={{
        display: "flex",
        flexDirection: "column",
        background: colorBgContainer,
        height: "calc(100% - 16px)",
        padding: 16,
      }}
    >
      <Space style={{ marginBottom: 16 }}>
        <Button
          type="primary"
          icon={<BranchesOutlined />}
          onClick={() => storeId && navigate(`/data/${storeId}/clean`)}
        >
          打开清洗工作台
        </Button>
      </Space>
      <div
        ref={(ref) => (containerRef.current = ref)}
        style={{
          flex: 1,
          minHeight: 0,
          width: "100%",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        {page.content?.length ? (
          <List style={{ width: "100%", height: "100%" }}>
            <VirtualList
              data={page.content}
              height={containerHeight}
              itemHeight={50}
              itemKey="id"
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
    </div>
  );
};

export default DataDetail;
