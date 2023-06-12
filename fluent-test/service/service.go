package service

import (
	"context"
	demopb "fluent-test/proto"
	"fmt"
	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
	"google.golang.org/protobuf/types/known/timestamppb"
	"io"
	"time"
)

type demoServiceImpl struct {
	demopb.UnimplementedDemoServiceServer
}

func NewDemoServiceImpl() demopb.DemoServiceServer {
	return new(demoServiceImpl)
}

func (d demoServiceImpl) Test(ctx context.Context, request *demopb.TestRequest) (*demopb.TestResponse, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if ok {
		value := md.Get("test-name")
		if len(value) > 0 {
			fmt.Println("test-name:", value[0])
		}
	}
	fmt.Println(fmt.Sprintf("Test request:%v", request))
	time.Sleep(time.Second * 2)
	md = metadata.Pairs("test-server-md", "test-server-md-value")
	_ = grpc.SendHeader(ctx, md)
	return &demopb.TestResponse{
		Name:           "",
		Age:            request.Age + 10,
		CommonMessage:  &demopb.CommonMessage{Id: 123},
		TestOneof:      &demopb.TestResponse_Gaga{Gaga: "gaga"},
		StartTime:      timestamppb.Now(),
		SubMchId:       nil,
		Attrs:          map[string]*demopb.CommonMessage{"testMap": {Id: 888}},
		List:           []string{"aaa", "bbb"},
		ResultCodeList: []demopb.ResultCode{demopb.ResultCode_INTERNAL_ERROR},
		ResultCode:     demopb.ResultCode_OK,
		Image:          []byte{0xF, 0xF},
	}, nil
}

func (d demoServiceImpl) TestNoPkg(ctx context.Context, request *demopb.TestNoPkgRequest) (*demopb.TextNoPkgResponse, error) {
	fmt.Println(fmt.Sprintf("TestNoPkg request:%v", request))
	time.Sleep(time.Second * 2)
	return &demopb.TextNoPkgResponse{
		ResultCode: demopb.ResultCode_OK,
		Name:       "TestNoPkg 李四",
		Age:        request.Age + 10,
	}, nil
}

func (d demoServiceImpl) TestSamePkg(ctx context.Context, request *demopb.TestSamePkgRequest) (*demopb.TestSamePkgResponse, error) {
	fmt.Println(fmt.Sprintf("TestSamePkg request:%v", request))
	time.Sleep(time.Second * 2)
	return &demopb.TestSamePkgResponse{
		ResultCode: demopb.ResultCode_OK,
		Name:       "TestSamePkg 李四",
		Age:        request.Age + 10,
	}, nil
}

func (d demoServiceImpl) ClientStream(stream demopb.DemoService_ClientStreamServer) error {
	for {
		req, err := stream.Recv()
		if err != nil {
			if err == io.EOF {
				return stream.SendAndClose(&demopb.StreamResponse{
					ResultCode: 0,
					Name:       "serverName",
					Age:        100,
				})
			}
			fmt.Println("stream recv error:", err)
			return stream.SendAndClose(&demopb.StreamResponse{
				ResultCode: demopb.ResultCode_INTERNAL_ERROR,
				Name:       "",
				Age:        0,
			})
		}
		fmt.Println("req:", req)
	}
}

func (d demoServiceImpl) ServerStream(req *demopb.StreamRequest, stream demopb.DemoService_ServerStreamServer) error {
	return nil
}

func (d demoServiceImpl) FullStream(stream demopb.DemoService_FullStreamServer) error {
	return nil
}
