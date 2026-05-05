import Icon from "@ant-design/icons";
import { Button, Modal } from "antd";
import { ReactComponent as Oracle } from "assets/svg/Oracle.svg";
import { ReactComponent as PostgreSQL } from "assets/svg/PostgreSQL.svg";
import { ReactComponent as JDBC } from "assets/svg/database.svg";
import { ReactComponent as MongoDB } from "assets/svg/mongodb.svg";
import { ReactComponent as MySQL } from "assets/svg/mysql.svg";
import { ReactComponent as SQLServer } from "assets/svg/sqlserver.svg";

import { StoreType } from "types/DB";
import "./index.css";

const typeMap = {
  [StoreType.MySQL]: MySQL,
  [StoreType.Postgresql]: PostgreSQL,
  [StoreType.Oracle]: Oracle,
  [StoreType.SQLServer]: SQLServer,
  [StoreType.MongoDB]: MongoDB,
  [StoreType.RDB]: JDBC,
};

export const selectDB = async (): Promise<StoreType> => {
  return new Promise((resolve) => {
    const handleBtnClick = (value: StoreType) => {
      resolve(value);
      modal.destroy();
    };
    const modal = Modal.confirm({
      title: "请选择数据库类型",
      closable: true,
      width: 400,
      footer: null,
      className: "plain-modal",
      content: (
        <div
          style={{
            display: "flex",
            flexWrap: "wrap",
            justifyContent: "space-around",
          }}
        >
          {Object.entries(typeMap).map(([type, icon]) => (
            <Button
              type="text"
              icon={<Icon component={icon} />}
              onClick={() => handleBtnClick(type as StoreType)}
              style={{ minWidth: 120 }}
            >
              {type}
            </Button>
          ))}
        </div>
      ),
    });
  });
};
