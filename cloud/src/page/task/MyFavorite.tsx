import { useEffect, useState } from "react";
import {
  deletefavoriteTemplate,
  favoriteTemplate,
  queryFavoriteTemplate,
} from "api/template";
import { type Page } from "types/Page";
import { type Template } from "types/Template";

import { InfiniteTemplateList } from "./TemplateList";

const MyFavorite = ({ card }: { card: boolean }) => {
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState({} as Page<Template>);

  const query = async (number: number) => {
    setLoading(true);
    try {
      const data = await queryFavoriteTemplate(number, 12);
      setPage(data as Page<Template>);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  useEffect(() => {
    query(0);
  }, []);

  const handleLike = async (id: string, like: boolean) => {
    if (like) {
      await favoriteTemplate(id);
    } else {
      await deletefavoriteTemplate(id);
    }
    query(0);
  };

  const loadMoreData = async () => {
    if (loading) {
      return;
    }
    setLoading(true);
    try {
      const data = await queryFavoriteTemplate(page.number + 1);
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
    <div style={{ display: "flex", height: "100%" }}>
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

export default MyFavorite;
