const { createProxyMiddleware } = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
    "/octopus",
    createProxyMiddleware({
      target: "http://localhost:8080/api",
      changeOrigin: true,
      pathRewrite: {
        "^/octopus/": "/"
      }
    })
  );
};
