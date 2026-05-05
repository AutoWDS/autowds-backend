import { message } from "antd";
import _ from "lodash";
import { objectToSearchString } from "use-query-params";

export const BASE_URL = process.env.REACT_APP_API_BASE_URL;

function absoluteUrl(url: string, params?: any): string {
  url = _.first(url) === "/" ? url : "/" + url;
  const search = params ? `?${objectToSearchString(params)}` : "";
  return `${BASE_URL}${url}${search}`;
}

interface Interceptor {
  request(ajax: Ajax): Promise<void>;
  response<T>(response: Response): Promise<T>;
}

let globalInterceptor: Interceptor | null;

export function setupInterceptor(i: Interceptor) {
  globalInterceptor = i;
}

export function removeInterceptor() {
  globalInterceptor = null;
}

export class Ajax {
  url: string;
  params: any;
  body: any;
  headers: any;
  method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH" = "GET";
  constructor(url: string) {
    this.url = url;
    this.headers = {
      "Content-Type": "application/json",
    };
  }

  path(child: string) {
    let baseUrl = this.url;
    if (!baseUrl) {
      this.url = child;
      return this;
    }
    if (_.last(baseUrl) === "/") {
      baseUrl = baseUrl.slice(0, -1);
    }
    this.url = _.head(child) === "/" ? baseUrl + child : baseUrl + "/" + child;
    return this;
  }

  query(queryObj: any) {
    this.params = queryObj;
    return this;
  }

  payload(payload: any) {
    this.body = JSON.stringify(payload);
    return this;
  }

  form(body: string) {
    this.body = body;
    return this;
  }

  addHeader(key: string, value: string) {
    this.headers = _.assign(this.headers, { [key]: value });
    return this;
  }

  get() {
    this.method = "GET";
    return this.fetch();
  }

  post() {
    this.method = "POST";
    return this.fetch();
  }

  delete() {
    this.method = "DELETE";
    return this.fetch();
  }

  put() {
    this.method = "PUT";
    return this.fetch();
  }

  patch() {
    this.method = "PATCH";
    return this.fetch();
  }

  async fetch() {
    if (globalInterceptor) {
      await globalInterceptor.request(this);
    }
    const r = await this.innerFetch();
    if (globalInterceptor) {
      return globalInterceptor.response(r);
    } else {
      return r;
    }
  }

  private async innerFetch() {
    const { url, params, ...rest } = this;
    try {
      return await fetch(absoluteUrl(url, params), rest);
    } catch (e) {
      message.error("请检查网络是否正常");
      throw e;
    }
  }
}

export default function ajax(url: string) {
  return new Ajax(url);
}
