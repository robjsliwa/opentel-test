package main

import (
	"context"
	"fmt"
	"log"

	name "opentel-test/nameserver/proto"

	"go.opentelemetry.io/otel/api/global"
	"go.opentelemetry.io/otel/api/kv"
	"go.opentelemetry.io/otel/api/trace"
	"go.opentelemetry.io/otel/exporters/trace/jaeger"
	"go.opentelemetry.io/otel/plugin/grpctrace"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"

	"google.golang.org/grpc"
)

func main() {
	tp := initTraceProvider("SidekickNameClient")
	defer tp()

	tracer := global.Tracer("SidekickNameClient")
	cc, err := grpc.Dial("[::1]:60000", grpc.WithInsecure(), grpc.WithUnaryInterceptor(grpctrace.UnaryClientInterceptor(tracer)))

	if err != nil {
		log.Fatalf("Connection error: %v", err)
	}

	defer cc.Close()

	c := name.NewNameClient(cc)
	getNickname(c, tracer)
}

func initTraceProvider(service string) func() {
	_, flush, err := jaeger.NewExportPipeline(
		jaeger.WithCollectorEndpoint("http://localhost:14268/api/traces"),
		jaeger.WithProcess(jaeger.Process{
			ServiceName: service,
			Tags: []kv.KeyValue{
				kv.Key("exporter").String("jaeger"),
			},
		}),
		jaeger.RegisterAsGlobal(),
		jaeger.WithSDK((&sdktrace.Config{DefaultSampler: sdktrace.AlwaysSample()})),
	)

	if err != nil {
		log.Fatal(err)
	}

	return func() {
		flush()
	}
}

func getNickname(c name.NameClient, tracer trace.Tracer) {
	ctx := context.Background()
	ctx, span := tracer.Start(ctx, "GetSidekickNickname")
	defer span.End()
	
	req := &name.NameRequest {}
	res, err := c.GetName(ctx, req)
	if err != nil {
		span.RecordError(ctx, err)
		log.Fatal("Could not get nickname.")
		return
	}

	span.AddEvent(ctx, "Response", kv.String("SidekickName", res.Name))
	fmt.Printf("Nickname: %v\n", res.Name)
}
