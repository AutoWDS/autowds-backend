import { HomeOutlined } from "@ant-design/icons";
import { Breadcrumb, type BreadcrumbProps } from "antd";
import _ from "lodash";
import { useLocation } from "react-router";
import { Link } from "react-router-dom";
import { mainMenu } from "./App";

type BreadcrumbItem = NonNullable<BreadcrumbProps["items"]>[number];

const home = {
  title: (
    <Link to="/">
      <HomeOutlined />
    </Link>
  ),
};

const breadcrumbNameMap: { [path: string]: string } = {
  "/task/myself": "我的任务",
  "/task/template": "模板市场",
  "/task/favorite": "我的收藏",
  "/task/myself/settings": "调度设置",
  "/task/myself/instances": "调度记录",
  "/task/myself/quality": "数据质量",
  "/task/myself/instances/data": "数据",
  "/task/myself/instances/log": "日志",
  "/data/store": "云端存储",
  "/data/db": "数据连接",
  "/data/sync": "数据同步",
  "/data/db/new/MySQL": "新建MySQL连接",
  "/data/db/new/Postgresql": "新建PostgreSQL连接",
  "/data/db/new/Oracle": "新建Oracle连接",
  "/data/db/new/SQLServer": "新建SQLServer连接",
  "/data/db/new/MongoDB": "新建MongoDB连接",
  "/data/db/new/RDB": "新建RDB连接",
  "/data/store/detail": "数据详情",
  "/pay": "升级会员",
};

function computeBreadcrumb(pathSnippets: string[]): BreadcrumbItem[] {
  if (!pathSnippets.length) {
    return [home];
  }
  console.log(pathSnippets);
  const extraBreadcrumbItems = _.chain(pathSnippets)
    .map((path, index) => {
      if (index === 0) {
        const { icon, label } =
          _.chain(mainMenu)
            .filter(({ key }) => key === path)
            .head()
            .value() || {};
        return {
          title: (
            <>
              {icon}
              {label}
            </>
          ),
        };
      }
      if (path.match(/^\d+$/g)) {
        return undefined;
      }
      const simpleURL = `/${pathSnippets
        .slice(0, index + 1)
        .filter((p) => !p.match(/^\d+$/g))
        .join("/")}`;
      const title = breadcrumbNameMap[simpleURL];
      if (!title) {
        return undefined;
      }
      const url = `/${pathSnippets.slice(0, index + 1).join("/")}`;
      console.log(simpleURL, url);
      return {
        key: url,
        title: <Link to={url}>{title}</Link>,
      };
    })
    .compact()
    .value();
  return [home].concat(extraBreadcrumbItems);
}

const Navigator = () => {
  const { pathname } = useLocation();
  const pathSnippets = pathname.split("/").filter((i) => i);
  const items = computeBreadcrumb(pathSnippets);
  return <Breadcrumb style={{ margin: "-10px 10px 10px" }} items={items} />;
};

export default Navigator;
