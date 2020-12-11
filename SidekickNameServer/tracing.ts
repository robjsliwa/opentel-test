import { LogLevel } from "@opentelemetry/core";
import { NodeTracerProvider } from "@opentelemetry/node";
import { SimpleSpanProcessor } from "@opentelemetry/tracing";
import { JaegerExporter } from "@opentelemetry/exporter-jaeger";

const trace_host = process.env.TRACE_HOST
  ? process.env.TRACE_HOST
  : "localhost";

const provider: NodeTracerProvider = new NodeTracerProvider({
  logLevel: LogLevel.ERROR,
});

provider.register();

provider.addSpanProcessor(
  new SimpleSpanProcessor(
    new JaegerExporter({
      serviceName: "SidekickNameServer",
      endpoint: `http://${trace_host}:14268/api/traces`,
      // If you are running your tracing backend on another host,
      // you can point to it using the `url` parameter of the
      // exporter config.
    })
  )
);

console.log("tracing initialized");
