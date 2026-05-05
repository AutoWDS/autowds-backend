import { Divider, List } from "antd";
import InfiniteScroll from "react-infinite-scroll-component";
import { type Page } from "types/Page";
import { type Template } from "types/Template";

import TemplateItem from "./TemplateItem";

import "./TemplateList.css";

interface TemplateListProps {
  loading?: boolean;
  skeleton?: boolean;
  dataSource?: Template[];
  card: boolean;
  onLike: (item: string, like: boolean, index: number) => void;
}

export const TemplateList = (props: TemplateListProps) => {
  const { loading, skeleton, card, dataSource, onLike } = props;
  return (
    <List
      loading={loading}
      grid={
        card
          ? { gutter: 8, column: 2, xs: 2, sm: 2, md: 2, lg: 3, xl: 3, xxl: 3 }
          : undefined
      }
      dataSource={dataSource}
      renderItem={(t, i) => (
        <TemplateItem
          index={i}
          loading={skeleton}
          template={t}
          card={card}
          onLike={onLike}
        />
      )}
    />
  );
};

interface InfiniteTemplateListProps extends TemplateListProps {
  page: Page<Template>;
  onLoadMore: () => any;
}

export const InfiniteTemplateList = (props: InfiniteTemplateListProps) => {
  const { loading, card, page, onLike, onLoadMore } = props;
  return (
    <div id="template-list-scrollable">
      <InfiniteScroll
        dataLength={page.content?.length || 0}
        next={onLoadMore}
        hasMore={!page.content || page.content.length < page.totalElements}
        loader={
          loading ? (
            <TemplateList
              card={card}
              skeleton={loading}
              dataSource={Array(6).fill({})}
              onLike={onLike}
            />
          ) : null
        }
        endMessage={
          page.content?.length > 10 ? (
            <Divider plain>
              {page.content?.length < page.totalElements
                ? "疯狂加载中..."
                : "我们将开发出更多模板，敬请期待"}
            </Divider>
          ) : null
        }
        scrollableTarget="template-list-scrollable"
      >
        <TemplateList
          loading={loading}
          card={card}
          dataSource={page.content}
          onLike={onLike}
        />
      </InfiniteScroll>
    </div>
  );
};
