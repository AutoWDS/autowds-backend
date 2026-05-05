import { ConfigProvider } from "antd";
import zhCN from "antd/locale/zh_CN";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { QueryParamProvider } from "use-query-params";
import { ReactRouter6Adapter } from "use-query-params/adapters/react-router-6";
import AjaxInterceptor from "utils/AjaxInterceptor";
import App from "./App";
import { Login } from "./components/account/Login";
import reportWebVitals from "./reportWebVitals";
import { Register } from "components/account/Register";
import { ResetPasswd } from "components/account/ResetPasswd";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

const Root = () => {
  return (
    <ConfigProvider locale={zhCN}>
      <BrowserRouter basename={process.env.PUBLIC_URL}>
        <AjaxInterceptor />
        <QueryParamProvider adapter={ReactRouter6Adapter}>
          <Routes>
            <Route path="/login" Component={Login} />
            <Route path="/user/register" Component={Register} />
            <Route path="/user/reset-passwd" Component={ResetPasswd} />
            <Route path="/*" Component={App} />
          </Routes>
        </QueryParamProvider>
      </BrowserRouter>
    </ConfigProvider>
  );
};

root.render(<Root />);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
