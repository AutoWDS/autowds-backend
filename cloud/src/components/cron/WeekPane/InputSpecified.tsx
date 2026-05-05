import { Checkbox, Col, Row, Space, type GetProp } from "antd"
import { useMemo } from "react"

type CheckboxValue = NonNullable<GetProp<typeof Checkbox.Group, "value">>[number]

const weekOptions = {
  SUN: "周日",
  MON: "周一",
  TUE: "周二",
  WED: "周三",
  THU: "周四",
  FRI: "周五",
  SAT: "周六"
}

interface InputSpecifiedProps {
  disabled: boolean
  value: string
  onChange: (value: string) => void
}
function InputSpecified(props: InputSpecifiedProps) {
  const { disabled, value, onChange } = props
  let selected: string[] = []
  if (!disabled) {
    selected = value.split(",")
  }
  const onChangeSelected = (v: CheckboxValue[]) =>
    onChange(v.length === 0 ? "SUN" : v.join(","))

  const checkList = useMemo(() => {
    return Object.entries(weekOptions).map(([weekCode, weekName]) => {
      return (
        <Col key={weekCode} flex={1}>
          <Checkbox disabled={disabled} value={weekCode}>
            {weekName}
          </Checkbox>
        </Col>
      )
    })
  }, [disabled])

  return (
    <Space>
      <span>指定</span>
      <Checkbox.Group
        style={{ width: 450 }}
        value={selected}
        onChange={onChangeSelected}>
        <Row>{checkList}</Row>
      </Checkbox.Group>
    </Space>
  )
}

export default InputSpecified
