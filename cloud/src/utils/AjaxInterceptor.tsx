import { message } from "antd";
import { useEffect, type PropsWithChildren } from "react";
import { useNavigate } from "react-router-dom";

import { Ajax, removeInterceptor, setupInterceptor } from "./ajax";
import { getAuthUser } from "api/user";

/**
 * RFC7807
 */
interface ProblemDetail {
  detail: string;
  instance: string;
  status: number;
}

export class RequestError {
  problem: ProblemDetail;
  constructor(problem: ProblemDetail) {
    this.problem = problem;
  }
}

const AjaxInterceptor = ({ children }: PropsWithChildren) => {
  const navigate = useNavigate();

  useEffect(() => {
    setupInterceptor({
      request: async (ajax: Ajax) => {
        const user = await getAuthUser()
        const token = user?.token;
        if (token) {
          ajax.headers = {
            ...ajax.headers,
            Authorization: `Bearer ${token}`,
          };
        }
      },
      response: async (r: Response) => {
        if (r.ok) {
          return r.json();
        }
        const problem = (await r.json()) as ProblemDetail;
        const { detail = "请求失败", instance } = problem;
        if (r.status === 401 && instance !== "/token") {
          navigate("/login");
          return;
        } else {
          message.error(detail);
        }
        throw new RequestError(problem);
      },
    });
    return () => removeInterceptor();
  }, [navigate]);
  return <>{children}</>;
};

export default AjaxInterceptor;
