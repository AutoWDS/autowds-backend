import type { Page } from "types/Page";
import type { Favorite, Template, TemplateTopic } from "types/Template";
import ajax from "utils/ajax";

const template = () => ajax("/template");

export interface TemplateFilter {
  name: string;
  topic: TemplateTopic;
  page?: number;
  size?: number;
  sort?: "modified" | "favCount";
}

export async function queryTaskTemplate(
  filter: TemplateFilter
): Promise<Page<Template>> {
  return template().query(filter).get() as Promise<Page<Template>>;
}

export async function queryFavoriteTemplate(
  page: number,
  size: number = 10
): Promise<Page<Template>> {
  return template().path("/favorite").query({ page, size }).get() as Promise<
    Page<Template>
  >;
}

export async function favoriteTemplate(templateId: string): Promise<Favorite> {
  return template()
    .path(templateId)
    .path("/favorite")
    .post() as Promise<Favorite>;
}

export async function deletefavoriteTemplate(
  templateId: string
): Promise<void> {
  await template().path(templateId).path("/favorite").delete();
}
