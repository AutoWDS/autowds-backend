import {
  CheckCircleOutlined,
  CreditCardOutlined,
  CrownOutlined,
  WechatOutlined,
  AlipayCircleOutlined,
} from "@ant-design/icons";
import {
  Button,
  Card,
  Col,
  Divider,
  Modal,
  QRCode,
  Radio,
  Row,
  Space,
  Spin,
  Tag,
  Typography,
  message,
} from "antd";
import { useEffect, useRef, useState } from "react";
import { createOrder, OrderLevel, PayFrom, queryOrderStatus } from "api/pay";
import { currentUser } from "api/user";
import { ProductEdition } from "types/Template";
import { StringParam, useQueryParams } from "use-query-params";

const { Title, Text, Paragraph } = Typography;

type PlanType = "personal" | "professional";
type BillingType = "monthly" | "yearly";

interface PlanConfig {
  key: PlanType;
  name: string;
  description: string;
  monthlyPrice: number;
  yearlyPrice: number;
  features: string[];
  popular?: boolean;
}

const planConfigs: PlanConfig[] = [
  {
    key: "personal",
    name: "个人版",
    description: "适合个人用户，按月或按年灵活订阅",
    monthlyPrice: 29,
    yearlyPrice: 300,
    features: [
      "100个采集任务",
      "每日 1000 条数据",
      "基础数据导出功能",
      "邮件技术支持",
    ],
  },
  {
    key: "professional",
    name: "专业版",
    description: "适合专业用户，更多云端采集额度",
    monthlyPrice: 99,
    yearlyPrice: 960,
    popular: true,
    features: [
      "无限本地采集任务",
      "每日无限次数据导出",
      "高级数据导出功能",
      "优先邮件技术支持",
      "专属功能内测资格",
    ],
  },
];

function billingToOrderLevel(billing: BillingType): OrderLevel {
  return billing === "yearly" ? "Annual" : "Monthly";
}

function planToEdition(plan: PlanType): ProductEdition {
  return plan === "professional" ? "L2" : "L1";
}

const ALIPAY_QR_ICON_PATH =
  "M308.6 545.7c-19.8 2-57.1 10.7-77.4 28.6-61 53-24.5 150 99 150 71.8 0 143.5-45.7 199.8-119-80.2-38.9-148.1-66.8-221.4-59.6zm460.5 67c100.1 33.4 154.7 43 166.7 44.8A445.9 445.9 0 00960 512c0-247.4-200.6-448-448-448S64 264.6 64 512s200.6 448 448 448c155.9 0 293.2-79.7 373.5-200.5-75.6-29.8-213.6-85-286.8-120.1-69.9 85.7-160.1 137.8-253.7 137.8-158.4 0-212.1-138.1-137.2-229 16.3-19.8 44.2-38.7 87.3-49.4 67.5-16.5 175 10.3 275.7 43.4 18.1-33.3 33.4-69.9 44.7-108.9H305.1V402h160v-56.2H271.3v-31.3h193.8v-80.1s0-13.5 13.7-13.5H557v93.6h191.7v31.3H557.1V402h156.4c-15 61.1-37.7 117.4-66.2 166.8 47.5 17.1 90.1 33.3 121.8 43.9z";

const WECHAT_QR_ICON_PATH =
  "M690.1 377.4c5.9 0 11.8.2 17.6.5-24.4-128.7-158.3-227.1-319.9-227.1C209 150.8 64 271.4 64 420.2c0 81.1 43.6 154.2 111.9 203.6a21.5 21.5 0 019.1 17.6c0 2.4-.5 4.6-1.1 6.9-5.5 20.3-14.2 52.8-14.6 54.3-.7 2.6-1.7 5.2-1.7 7.9 0 5.9 4.8 10.8 10.8 10.8 2.3 0 4.2-.9 6.2-2l70.9-40.9c5.3-3.1 11-5 17.2-5 3.2 0 6.4.5 9.5 1.4 33.1 9.5 68.8 14.8 105.7 14.8 6 0 11.9-.1 17.8-.4-7.1-21-10.9-43.1-10.9-66 0-135.8 132.2-245.8 295.3-245.8zm-194.3-86.5c23.8 0 43.2 19.3 43.2 43.1s-19.3 43.1-43.2 43.1c-23.8 0-43.2-19.3-43.2-43.1s19.4-43.1 43.2-43.1zm-215.9 86.2c-23.8 0-43.2-19.3-43.2-43.1s19.3-43.1 43.2-43.1 43.2 19.3 43.2 43.1-19.4 43.1-43.2 43.1zm586.8 415.6c56.9-41.2 93.2-102 93.2-169.7 0-124-120.8-224.5-269.9-224.5-149 0-269.9 100.5-269.9 224.5S540.9 847.5 690 847.5c30.8 0 60.6-4.4 88.1-12.3 2.6-.8 5.2-1.2 7.9-1.2 5.2 0 9.9 1.6 14.3 4.1l59.1 34c1.7 1 3.3 1.7 5.2 1.7a9 9 0 006.4-2.6 9 9 0 002.6-6.4c0-2.2-.9-4.4-1.4-6.6-.3-1.2-7.6-28.3-12.2-45.3-.5-1.9-.9-3.8-.9-5.7.1-5.9 3.1-11.2 7.6-14.5zM600.2 587.2c-19.9 0-36-16.1-36-35.9 0-19.8 16.1-35.9 36-35.9s36 16.1 36 35.9c0 19.8-16.2 35.9-36 35.9zm179.9 0c-19.9 0-36-16.1-36-35.9 0-19.8 16.1-35.9 36-35.9s36 16.1 36 35.9a36.08 36.08 0 01-36 35.9z";

function getPaymentQrIcon(payFrom: PayFrom): string {
  const color = payFrom === "Alipay" ? "#1677ff" : "#52c41a";
  const path = payFrom === "Alipay" ? ALIPAY_QR_ICON_PATH : WECHAT_QR_ICON_PATH;
  const svg = `
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="64 64 896 896">
      <path fill="${color}" d="${path}" />
    </svg>
  `;

  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
}

const Pay = () => {
  const [query, setQuery] = useQueryParams({
    plan: StringParam,
    billing: StringParam,
  });

  const initialPlan: PlanType =
    query.plan === "professional" ? "professional" : "personal";
  const initialBilling: BillingType =
    query.billing === "monthly" ? "monthly" : "yearly";

  const [selectedPlan, setSelectedPlan] = useState<PlanType>(initialPlan);
  const [billing, setBilling] = useState<BillingType>(initialBilling);
  const [payFrom, setPayFrom] = useState<PayFrom>("Alipay");
  const [loading, setLoading] = useState(false);
  const [qrcodeUrl, setQrcodeUrl] = useState<string>("");
  const [payStatus, setPayStatus] = useState<string>("");
  const [modalOpen, setModalOpen] = useState(false);
  const pollTimer = useRef<ReturnType<typeof setInterval> | null>(null);

  const currentPlan = planConfigs.find((p) => p.key === selectedPlan)!;
  const actualPrice =
    billing === "yearly" ? currentPlan.yearlyPrice : currentPlan.monthlyPrice;

  useEffect(() => {
    setQuery({ plan: selectedPlan, billing }, "replaceIn");
  }, [selectedPlan, billing, setQuery]);

  useEffect(() => {
    return () => {
      if (pollTimer.current) {
        clearInterval(pollTimer.current);
      }
    };
  }, []);

  const startPolling = (oid: number) => {
    if (pollTimer.current) {
      clearInterval(pollTimer.current);
    }
    pollTimer.current = setInterval(async () => {
      try {
        const resp = await queryOrderStatus(oid);
        setPayStatus(resp.status);
        if (resp.status === "paid") {
          if (pollTimer.current) {
            clearInterval(pollTimer.current);
          }
          message.success("支付成功！");
          setTimeout(() => {
            setModalOpen(false);
            setQrcodeUrl("");
            setPayStatus("");
            currentUser().then(() => {
              window.location.reload();
            });
          }, 2000);
        } else if (resp.status === "closed") {
          if (pollTimer.current) {
            clearInterval(pollTimer.current);
          }
          message.error("订单已关闭");
          setModalOpen(false);
        }
      } catch (e) {
        console.error("查询订单状态失败:", e);
      }
    }, 3000);
  };

  const handlePay = async () => {
    setLoading(true);
    try {
      const level = billingToOrderLevel(billing);
      const edition = planToEdition(selectedPlan);
      const resp = await createOrder(level, edition, payFrom);
      if (resp.pay_from === "Paddle") {
        window.location.href = resp.qrcode_url;
        return;
      }
      setQrcodeUrl(resp.qrcode_url);
      setPayStatus("created");
      setModalOpen(true);
      startPolling(resp.order_id);
    } catch (e: any) {
      message.error("创建订单失败，请稍后重试");
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleModalClose = () => {
    if (pollTimer.current) {
      clearInterval(pollTimer.current);
    }
    setModalOpen(false);
    setQrcodeUrl("");
    setPayStatus("");
  };

  const handlePlanChange = (planKey: PlanType) => {
    setSelectedPlan(planKey);
  };

  const handleBillingChange = (billingType: BillingType) => {
    setBilling(billingType);
  };

  return (
    <div style={{ height: "100%", overflow: "auto" }}>
      <div style={{ padding: "24px", maxWidth: 1200, margin: "0 auto" }}>
        <div style={{ textAlign: "center", margin: "-40px 0 40px" }}>
          <Title level={2}>
            <CrownOutlined style={{ color: "#faad14", marginRight: 8 }} />
            升级会员
          </Title>
          <Paragraph type="secondary" style={{ fontSize: 16 }}>
            选择适合您的会员方案，解锁更多高级功能
          </Paragraph>
        </div>

        {/* Billing Toggle */}
        <div style={{ textAlign: "center", marginBottom: 32 }}>
          <div
            style={{
              display: "inline-flex",
              background: "#f5f5f5",
              borderRadius: 999,
              padding: 4,
            }}
          >
            <button
              onClick={() => handleBillingChange("monthly")}
              style={{
                height: 52,
                padding: "0 24px",
                borderRadius: 999,
                border: "none",
                cursor: "pointer",
                fontWeight: 500,
                transition: "all 0.2s",
                background: billing === "monthly" ? "#fff" : "transparent",
                color: billing === "monthly" ? "#262626" : "#8c8c8c",
                boxShadow:
                  billing === "monthly" ? "0 1px 2px rgba(0,0,0,0.05)" : "none",
              }}
            >
              按月付费
            </button>
            <button
              onClick={() => handleBillingChange("yearly")}
              style={{
                height: 52,
                padding: "0 24px",
                borderRadius: 999,
                border: "none",
                cursor: "pointer",
                fontWeight: 500,
                transition: "all 0.2s",
                background: billing === "yearly" ? "#fff" : "transparent",
                color: billing === "yearly" ? "#262626" : "#8c8c8c",
                boxShadow:
                  billing === "yearly" ? "0 1px 2px rgba(0,0,0,0.05)" : "none",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <span>按年付费</span>
              {billing === "yearly" && (
                <span
                  style={{ fontSize: 12, color: "#52c41a", fontWeight: 400 }}
                >
                  更优惠
                </span>
              )}
            </button>
          </div>
        </div>

        {/* Plan Selection */}
        <Row gutter={[24, 24]} justify="center" style={{ marginBottom: 24 }}>
          {planConfigs.map((plan) => {
            const planPrice =
              billing === "yearly" ? plan.yearlyPrice : plan.monthlyPrice;
            return (
              <Col xs={24} sm={12} md={10} key={plan.key}>
                <Card
                  hoverable
                  style={{
                    height: "100%",
                    borderColor:
                      selectedPlan === plan.key ? "#1890ff" : undefined,
                    boxShadow:
                      selectedPlan === plan.key
                        ? "0 0 0 2px #1890ff"
                        : undefined,
                  }}
                  onClick={() => handlePlanChange(plan.key)}
                  title={
                    <Space>
                      <Text strong style={{ fontSize: 18 }}>
                        {plan.name}
                      </Text>
                      {plan.popular && <Tag color="red">最受欢迎</Tag>}
                    </Space>
                  }
                >
                  <Paragraph type="secondary">{plan.description}</Paragraph>

                  {/* Price Display */}
                  <div style={{ textAlign: "center", margin: "16px 0" }}>
                    <Text
                      style={{
                        fontSize: 32,
                        fontWeight: "bold",
                        color: "#1890ff",
                      }}
                    >
                      ¥{planPrice}
                    </Text>
                    <Text
                      type="secondary"
                      style={{ marginLeft: 4, fontSize: 14 }}
                    >
                      {billing === "yearly" ? "/年" : "/月"}
                    </Text>
                    {billing === "yearly" && (
                      <div style={{ marginTop: 4 }}>
                        <Text type="secondary" delete style={{ fontSize: 12 }}>
                          ¥{plan.monthlyPrice * 12}/年
                        </Text>
                        <Tag
                          color="green"
                          style={{ marginLeft: 4, fontSize: 12 }}
                        >
                          省 ¥{plan.monthlyPrice * 12 - plan.yearlyPrice}
                        </Tag>
                      </div>
                    )}
                  </div>

                  <Divider style={{ margin: "16px 0" }} />
                  <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
                    {plan.features.map((feature, idx) => (
                      <li
                        key={idx}
                        style={{
                          marginBottom: 8,
                          display: "flex",
                          alignItems: "center",
                        }}
                      >
                        <CheckCircleOutlined
                          style={{ color: "#52c41a", marginRight: 8 }}
                        />
                        <Text>{feature}</Text>
                      </li>
                    ))}
                  </ul>
                </Card>
              </Col>
            );
          })}
        </Row>

        {/* Payment Method */}
        <Card style={{ maxWidth: 600, margin: "0 auto 24px" }}>
          <Title level={4} style={{ textAlign: "center", marginBottom: 24 }}>
            <CreditCardOutlined style={{ marginRight: 8 }} />
            支付方式
          </Title>
          <Radio.Group
            value={payFrom}
            onChange={(e) => setPayFrom(e.target.value)}
            style={{ width: "100%" }}
          >
            <Row gutter={[16, 16]}>
              <Col span={8}>
                <Radio.Button
                  value="Alipay"
                  style={{
                    width: "100%",
                    height: 64,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontSize: 16,
                  }}
                >
                  <AlipayCircleOutlined
                    style={{ color: "#1677ff", fontSize: 24, marginRight: 8 }}
                  />
                  支付宝
                </Radio.Button>
              </Col>
              <Col span={8}>
                <Radio.Button
                  value="Wechat"
                  style={{
                    width: "100%",
                    height: 64,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontSize: 16,
                  }}
                >
                  <WechatOutlined
                    style={{ color: "#52c41a", fontSize: 24, marginRight: 8 }}
                  />
                  微信支付
                </Radio.Button>
              </Col>
              <Col span={8}>
                <Radio.Button
                  value="Paddle"
                  style={{
                    width: "100%",
                    height: 64,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontSize: 16,
                  }}
                >
                  <CreditCardOutlined
                    style={{ color: "#722ed1", fontSize: 24, marginRight: 8 }}
                  />
                  Paddle
                </Radio.Button>
              </Col>
            </Row>
          </Radio.Group>

          <Divider />

          <div style={{ textAlign: "center" }}>
            <div style={{ marginBottom: 16 }}>
              <Text type="secondary">应付金额：</Text>
              <Text
                style={{ fontSize: 28, fontWeight: "bold", color: "#f5222d" }}
              >
                ¥{actualPrice}
              </Text>
            </div>
            <Button
              type="primary"
              size="large"
              loading={loading}
              onClick={handlePay}
              style={{ width: "100%", height: 48, fontSize: 18 }}
            >
              立即支付
            </Button>
          </div>
        </Card>

        <Modal
          open={modalOpen}
          title="扫码支付"
          onCancel={handleModalClose}
          width={400}
          footer={null}
          centered
        >
          <div style={{ textAlign: "center", padding: "20px 0" }}>
            {qrcodeUrl ? (
              <div
                style={{
                  width: 240,
                  height: 240,
                  margin: "0 auto",
                  border: "1px solid #d9d9d9",
                  borderRadius: 8,
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <QRCode
                  value={qrcodeUrl}
                  size={220}
                  icon={getPaymentQrIcon(payFrom)}
                  iconSize={40}
                  includeMargin
                  bordered={false}
                  status={payStatus === "paid" ? "scanned" : "active"}
                  statusRender={({ status }) =>
                    status === "scanned" ? (
                      <div style={{ textAlign: "center" }}>
                        <CheckCircleOutlined
                          style={{ fontSize: 64, color: "#52c41a" }}
                        />
                        <div style={{ marginTop: 16 }}>
                          <Text
                            strong
                            style={{ fontSize: 18, color: "#52c41a" }}
                          >
                            支付成功
                          </Text>
                        </div>
                      </div>
                    ) : null
                  }
                />
              </div>
            ) : (
              <Spin size="large" />
            )}
          </div>
        </Modal>
      </div>
    </div>
  );
};

export default Pay;
