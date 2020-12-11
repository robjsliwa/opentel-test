package main

import (
	"context"
	"log"
	"fmt"

	"opentel-test/nameserver/proto"
	"google.golang.org/grpc"
)

func main() {
	cc, err := grpc.Dial("[::1]:60000", grpc.WithInsecure())

	if err != nil {
		log.Fatalf("Connection error: %v", err)
	}

	c := name.NewNameClient(cc)
	getNickname(c)
}

func getNickname(c name.NameClient) {
	ctx := context.Background()
	
	req := &name.NameRequest {}
	res, err := c.GetName(ctx, req)
	if err != nil {
		log.Fatal("Could not get nickname.")
		return
	}

	fmt.Printf("Nickname: %v\n", res.Name)
}
