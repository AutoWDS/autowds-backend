import { Button, Form, Tooltip } from "antd";
import { selectCron } from "components/cron/Cron";
import cronstrue from "cronstrue";
import "cronstrue/locales/en";
import "cronstrue/locales/zh_CN";

interface Props {
  value?: string;
  onChange?: (value: string) => void;
}

const CronSelector = ({ value: cron, onChange }: Props) => {
  const { status } = Form.Item.useStatus();
  const tip = cron ? cronstrue.toString(cron, { locale: "zh_CN" }) : undefined;
  const handleSelectCron = async () => {
    const newCron = await selectCron(cron);
    onChange && onChange(newCron);
  };
  return (
    <Tooltip title={tip}>
      <Button
        danger={status === "error"}
        onClick={handleSelectCron}
        style={{ width: "100%" }}
      >
        {cron}
      </Button>
    </Tooltip>
  );
};

export default CronSelector;
