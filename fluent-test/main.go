package main

import (
	demopb "fluent-test/proto"
	"fluent-test/service"
	"fmt"
	"google.golang.org/grpc"
	"log"
	"net"
)

func main() {
	grpcServer := grpc.NewServer()
	demopb.RegisterDemoServiceServer(grpcServer, service.NewDemoServiceImpl())
	lis, err := net.Listen("tcp", ":25000")
	if err != nil {
		log.Panicf("net listen error : %v", err)
	}
	fmt.Println("Server start:", ":25000")
	if err := grpcServer.Serve(lis); err != nil {
		log.Panicf("serve error : %v", err)
	}
}
