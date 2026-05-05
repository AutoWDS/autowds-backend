import { Input, Select, Space } from "antd";
import {
  deletefavoriteTemplate,
  favoriteTemplate,
  queryTaskTemplate,
  type TemplateFilter,
} from "api/template";
import { produce } from "immer";
import _ from "lodash";
import { useEffect, useState } from "react";
import { type Page } from "types/Page";
import { TemplateTopicDesc, type Template } from "types/Template";

import i18n from "i18n";

import { InfiniteTemplateList } from "./TemplateList";

const siteTypeOptions = [
  { label: i18n("popup_commonTemplate_allTypes"), value: "" },
].concat(
  _.entries(TemplateTopicDesc).map(([key, value]) => ({
    label: i18n(value),
    value: key,
  }))
);

const orderOptions = [
  {
    label: i18n("popup_commonTemplate_sortByFavCount"),
    value: "favCount",
  },
  {
    label: i18n("popup_commonTemplate_sortByModified"),
    value: "modified",
  },
];

const CommonTemplate = ({ card }: { card: boolean }) => {
  const [filter, setFilter] = useState({ size: 12 } as TemplateFilter);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState({} as Page<Template>);

  const query = async (filter: TemplateFilter) => {
    setLoading(true);
    try {
      const data = await queryTaskTemplate(filter);
      setPage(data as Page<Template>);
    } catch (e) {
      console.log(e);
    }
    setLoading(false);
  };

  useEffect(() => {
    query(filter);
  }, [filter]);

  const handleLike = async (id: string, like: boolean, index: number) => {
    try {
      if (like) {
        await favoriteTemplate(id);
      } else {
        await deletefavoriteTemplate(id);
      }
      setPage((page) =>
        produce(page, (draft) => {
          _.update(draft.content, [index], ({ favCount, ...rest }) => ({
            ...rest,
            like,
            favCount: like ? favCount + 1 : favCount - 1,
          }));
        })
      );
    } catch (e) {}
  };

  const loadMoreData = async () => {
    if (loading) {
      return;
    }
    setLoading(true);
    try {
      const data = await queryTaskTemplate({
        ...filter,
        page: page.number + 1,
      });
      setPage((oldPage) => {
        const page = data as Page<Template>;
        page.content = oldPage.content.concat(page.content);
        return page;
      });
    } catch (e) {
      console.log(e);
    }
    setLoading(false);
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div
        style={{
          display: "flex",
          marginBottom: 16,
        }}
      >
        <Space>
          <Select
            placeholder={i18n("popup_commonTemplate_filterType")}
            options={siteTypeOptions}
            onChange={(topic) => setFilter((s) => ({ ...s, topic }))}
          />
          <Input
            placeholder={i18n("popup_commonTemplate_filterName")}
            onChange={(e) => setFilter((s) => ({ ...s, name: e.target.value }))}
          />
        </Space>
        <div style={{ flex: 1 }} />
        <Select
          placeholder={i18n("popup_commonTemplate_sortType")}
          options={orderOptions}
          onChange={(sort) => setFilter((s) => ({ ...s, sort }))}
        />
      </div>
      <InfiniteTemplateList
        loading={loading}
        card={card}
        page={page}
        onLoadMore={loadMoreData}
        onLike={handleLike}
      />
    </div>
  );
};

export default CommonTemplate;
