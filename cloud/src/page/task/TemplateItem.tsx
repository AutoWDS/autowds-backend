import {
  AppstoreAddOutlined,
  EyeOutlined,
  HeartFilled,
  HeartOutlined,
} from "@ant-design/icons";
import {
  Avatar,
  Card,
  List,
  Skeleton,
  Space,
  Tag,
  Tooltip,
  Typography,
} from "antd";
import {
  ProductEditionColor,
  ProductEditionDesc,
  TemplateTopicDesc,
  type Template,
} from "types/Template";

import "./TemplateItem.css";

import i18n from "i18n";

interface TemplateItemProps {
  loading?: boolean;
  card: boolean;
  template: Template;
  index: number;
  onLike: (item: string, like: boolean, index: number) => void;
}

const TemplateItem = ({
  loading,
  card,
  template,
  index,
  onLike,
}: TemplateItemProps) => {
  const { id, name, detail, img, favCount, like, topic, edition } = template;
  const actions = [
    <Tooltip title={i18n("popup_commonTemplate_actions_preview")}>
      <EyeOutlined key="setting" />
    </Tooltip>,
    <Space onClick={() => onLike(id, !like, index)}>
      {like ? (
        <HeartFilled key="edit" style={{ color: "#eb2f96" }} />
      ) : (
        <HeartOutlined key="edit" />
      )}
      <span>{favCount}</span>
    </Space>,
    <Tooltip title={i18n("popup_commonTemplate_actions_apply")}>
      <AppstoreAddOutlined key="ellipsis" />
    </Tooltip>,
  ];
  const i18nTopic = TemplateTopicDesc[topic];
  const i18nEdition = ProductEditionDesc[edition];
  const title = (
    <Space direction={card ? "vertical" : "horizontal"}>
      <b>{name}</b>
      <Space size={2}>
        <Tag color="#108ee9">{i18nTopic ? i18n(i18nTopic) : null}</Tag>
        <Tag color={ProductEditionColor[edition]}>
          {i18nEdition ? i18n(i18nEdition) : null}
        </Tag>
      </Space>
    </Space>
  );
  const detailDescription = (
    <Typography.Paragraph
      ellipsis={{ rows: 2, tooltip: detail }}
      style={{ marginBottom: 0 }}
    >
      {detail}
    </Typography.Paragraph>
  );

  return card ? (
    <List.Item className="list-item-card">
      <Card actions={loading ? undefined : actions}>
        <Skeleton loading={loading} avatar={{ shape: "square" }} active>
          <Card.Meta
            avatar={<Avatar shape="square" size={64} src={img} />}
            title={title}
            description={detailDescription}
          />
        </Skeleton>
      </Card>
    </List.Item>
  ) : (
    <List.Item className="list-item" actions={loading ? undefined : actions}>
      <Skeleton
        loading={loading}
        avatar={{ shape: "square" }}
        paragraph={{ rows: 1 }}
        active
      >
        <List.Item.Meta
          avatar={<Avatar shape="square" size={48} src={img} />}
          title={title}
          description={detailDescription}
        />
      </Skeleton>
    </List.Item>
  );
};

export default TemplateItem;
