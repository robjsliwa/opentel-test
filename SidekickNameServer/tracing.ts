import { LogLevel } from "@opentelemetry/core";
import { NodeTracerProvider } from "@opentelemetry/node";
import { SimpleSpanProcessor } from "@opentelemetry/tracing";
import { JaegerExporter } from "@opentelemetry/exporter-jaeger";

const trace_host = process.env.TRACE_HOST
  ? process.env.TRACE_HOST
  : "localhost";

const provider: NodeTracerProvider = new NodeTracerProvider({
  plugins: {
    express: {
      enabled: false,
      path: "@opentelemetry/plugin-express",
    },
    http: {
      enabled: true,
      path: "@opentelemetry/plugin-http",
    },
  },
});

provider.addSpanProcessor(
  new SimpleSpanProcessor(
    new JaegerExporter({
      serviceName: "SidekickNameServer",
      endpoint: `http://${trace_host}:14268/api/traces`,
    })
  )
);

provider.register();

console.log("tracing initialized");
