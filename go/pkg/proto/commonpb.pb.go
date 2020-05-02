// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.21.0
// 	protoc        v3.9.2
// source: commonpb.proto

package proto

import (
	proto "github.com/golang/protobuf/proto"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// This is a compile-time assertion that a sufficiently up-to-date version
// of the legacy proto package is being used.
const _ = proto.ProtoPackageIsVersion4

type State int32

const (
	State_OK           State = 0
	State_WRONG_LEADER State = 1
	State_NOT_FOUND    State = 2
	State_IO_ERROR     State = 3
)

// Enum value maps for State.
var (
	State_name = map[int32]string{
		0: "OK",
		1: "WRONG_LEADER",
		2: "NOT_FOUND",
		3: "IO_ERROR",
	}
	State_value = map[string]int32{
		"OK":           0,
		"WRONG_LEADER": 1,
		"NOT_FOUND":    2,
		"IO_ERROR":     3,
	}
)

func (x State) Enum() *State {
	p := new(State)
	*p = x
	return p
}

func (x State) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (State) Descriptor() protoreflect.EnumDescriptor {
	return file_commonpb_proto_enumTypes[0].Descriptor()
}

func (State) Type() protoreflect.EnumType {
	return &file_commonpb_proto_enumTypes[0]
}

func (x State) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use State.Descriptor instead.
func (State) EnumDescriptor() ([]byte, []int) {
	return file_commonpb_proto_rawDescGZIP(), []int{0}
}

var File_commonpb_proto protoreflect.FileDescriptor

var file_commonpb_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x0d, 0x62, 0x61, 0x79, 0x61, 0x72, 0x64, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2a,
	0x3e, 0x0a, 0x05, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x4f, 0x4b, 0x10, 0x00,
	0x12, 0x10, 0x0a, 0x0c, 0x57, 0x52, 0x4f, 0x4e, 0x47, 0x5f, 0x4c, 0x45, 0x41, 0x44, 0x45, 0x52,
	0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10,
	0x02, 0x12, 0x0c, 0x0a, 0x08, 0x49, 0x4f, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x03, 0x42,
	0x0b, 0x5a, 0x09, 0x70, 0x6b, 0x67, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_commonpb_proto_rawDescOnce sync.Once
	file_commonpb_proto_rawDescData = file_commonpb_proto_rawDesc
)

func file_commonpb_proto_rawDescGZIP() []byte {
	file_commonpb_proto_rawDescOnce.Do(func() {
		file_commonpb_proto_rawDescData = protoimpl.X.CompressGZIP(file_commonpb_proto_rawDescData)
	})
	return file_commonpb_proto_rawDescData
}

var file_commonpb_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_commonpb_proto_goTypes = []interface{}{
	(State)(0), // 0: bayard.common.State
}
var file_commonpb_proto_depIdxs = []int32{
	0, // [0:0] is the sub-list for method output_type
	0, // [0:0] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_commonpb_proto_init() }
func file_commonpb_proto_init() {
	if File_commonpb_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_commonpb_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   0,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_commonpb_proto_goTypes,
		DependencyIndexes: file_commonpb_proto_depIdxs,
		EnumInfos:         file_commonpb_proto_enumTypes,
	}.Build()
	File_commonpb_proto = out.File
	file_commonpb_proto_rawDesc = nil
	file_commonpb_proto_goTypes = nil
	file_commonpb_proto_depIdxs = nil
}
