import { ProductEdition } from "types/Template";
import ajax from "utils/ajax";

export type OrderLevel = "Monthly" | "Annual";
export type PayFrom = "Alipay" | "Wechat" | "Paddle";
export type OrderStatus = "created" | "paid" | "closed";

export interface CreateOrderResponse {
  order_id: number;
  qrcode_url: string;
  pay_from: PayFrom;
}

export interface OrderStatusResponse {
  order_id: number;
  status: OrderStatus;
  level: OrderLevel;
  pay_from: PayFrom;
  created: string;
  confirm: string | null;
}

const pay = () => ajax("/pay");

export function createOrder(level: OrderLevel, edition: ProductEdition, payFrom: PayFrom): Promise<CreateOrderResponse> {
  return pay()
    .path("/create")
    .addHeader("Content-Type", "application/x-www-form-urlencoded")
    .form(`level=${level}&edition=${edition}&pay_from=${payFrom}`)
    .post() as Promise<CreateOrderResponse>;
}

export function queryOrderStatus(orderId: number): Promise<OrderStatusResponse> {
  return pay()
    .path(`/${orderId}/status`)
    .post() as Promise<OrderStatusResponse>;
}
