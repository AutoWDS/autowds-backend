import Icon from "@ant-design/icons";
import { Button, Modal } from "antd";
import { ReactComponent as CSVIcon } from "assets/svg/csv.svg";
import { ReactComponent as ExcelIcon } from "assets/svg/excel.svg";
import { ReactComponent as JsonIcon } from "assets/svg/json.svg";
import { ExporterType } from "types/ExporterType";

import "./index.css";

export const selectDataExporter = async (): Promise<ExporterType> => {
  return new Promise((resolve) => {
    const handleBtnClick = (value: ExporterType) => {
      resolve(value);
      modal.destroy();
    };
    const modal = Modal.confirm({
      title: "请选择导出文件类型",
      closable: true,
      width: 400,
      footer: null,
      className: "plain-modal",
      content: (
        <>
          <Button
            type="text"
            icon={<Icon component={ExcelIcon} />}
            onClick={() => handleBtnClick(ExporterType.excel)}
          >
            Excel
          </Button>
          <Button
            type="text"
            icon={<Icon component={CSVIcon} />}
            onClick={() => handleBtnClick(ExporterType.csv)}
          >
            CSV
          </Button>
          <Button
            type="text"
            icon={<Icon component={JsonIcon} />}
            onClick={() => handleBtnClick(ExporterType.csv)}
          >
            JSON
          </Button>
        </>
      ),
    });
  });
};
