// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package name

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion7

// NameClient is the client API for Name service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type NameClient interface {
	GetName(ctx context.Context, in *NameRequest, opts ...grpc.CallOption) (*NameResponse, error)
}

type nameClient struct {
	cc grpc.ClientConnInterface
}

func NewNameClient(cc grpc.ClientConnInterface) NameClient {
	return &nameClient{cc}
}

func (c *nameClient) GetName(ctx context.Context, in *NameRequest, opts ...grpc.CallOption) (*NameResponse, error) {
	out := new(NameResponse)
	err := c.cc.Invoke(ctx, "/name.Name/GetName", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// NameServer is the server API for Name service.
// All implementations must embed UnimplementedNameServer
// for forward compatibility
type NameServer interface {
	GetName(context.Context, *NameRequest) (*NameResponse, error)
	mustEmbedUnimplementedNameServer()
}

// UnimplementedNameServer must be embedded to have forward compatible implementations.
type UnimplementedNameServer struct {
}

func (UnimplementedNameServer) GetName(context.Context, *NameRequest) (*NameResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetName not implemented")
}
func (UnimplementedNameServer) mustEmbedUnimplementedNameServer() {}

// UnsafeNameServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to NameServer will
// result in compilation errors.
type UnsafeNameServer interface {
	mustEmbedUnimplementedNameServer()
}

func RegisterNameServer(s grpc.ServiceRegistrar, srv NameServer) {
	s.RegisterService(&_Name_serviceDesc, srv)
}

func _Name_GetName_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(NameRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(NameServer).GetName(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/name.Name/GetName",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(NameServer).GetName(ctx, req.(*NameRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _Name_serviceDesc = grpc.ServiceDesc{
	ServiceName: "name.Name",
	HandlerType: (*NameServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetName",
			Handler:    _Name_GetName_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "name.proto",
}