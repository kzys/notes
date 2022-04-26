# containerd's gogo/protobuf migration

containerd has migrated from
[github.com/gogo/protobuf](https://github.com/gogo/protobuf/) to
[google.golang.org/protobuf](https://github.com/protocolbuffers/protobuf-go) in April 2022.

This document describes the effort.

## Background

containerd has been using gogo/protobuf since the very beginning (TODO: when?). While I wasn't involving the original decision, using gogo/protobuf seemed popular at that time. For example, the following project are/were using gogo/protobuf and only 2/4 are migrated.

- [Vitess](https://github.com/vitessio/vitess) ([migrating off in 2021](https://vitess.io/blog/2021-06-03-a-new-protobuf-generator-for-go/))
- [Istio](https://github.com/istio/istio) ([migrating off in 2022](https://github.com/istio/istio/pull/38055))
- [etcd](https://github.com/etcd-io/etcd)
- [CockroachDB](https://github.com/cockroachdb/cockroach)

However, the maintainers of gogo/protobuf decided to step down and the project is [looking for new maintainers since May 2020](https://github.com/gogo/protobuf/issues/691).

## Protobuild

containerd is using [Protobuild](https://github.com/containerd/protobuild) that wraps protoc-gen-go. Previously protoc-gen-go supported gRPC through its "plugin" mechanism, but the recent versions of protoc-gen-go don't (TODO: when?). Now protoc-gen-go is only responsible for Protocol Buffers, and gRPC needs protoc-gen-go-grpc.

I extended Protobuild to support multiple generators.

- [Support multiple generators](https://github.com/containerd/protobuild/pull/45)
- [Remove plugins and import_path](https://github.com/containerd/protobuild/pull/48)

## ttrpc

ttrpc is containerd's own RPC protocol, which is basically "gRPC without HTTP".

I have removed gogo/protobuf from ttrpc, and written a new code generator that works more like protoc-gen-go-grpc.

- [Use google.golang.org/protobuf instead of github.com/gogo/protobuf](https://github.com/containerd/ttrpc/pull/99)
- [Add protoc-gen-go-ttrpc](https://github.com/containerd/ttrpc/pull/96)

## containerd

containerd has been using a lot of gogo/protobuf extensions. I have removed all of them in the following PRs. Luckily containerd wasn't passing these Protobuf-generated messages around and was converting them to internal domain types right after getting the messages. The blast radius of the changes was mitigated due to that.

- [Remove gogoproto.customtype](https://github.com/containerd/containerd/pull/6699)
- [Remove enumvalue_customname, goproto_enum_prefix and enum_customname](https://github.com/containerd/containerd/pull/6708)
- [Remove gogoproto.stdtime](https://github.com/containerd/containerd/pull/6821)
- [Remove all gogoproto extensions](https://github.com/containerd/containerd/pull/6829)
- [Consolidate gogo/protobuf dependencies under our own protobuf package](https://github.com/containerd/containerd/pull/6826)

Because of the PRs above, the size of the final migration PR was manageable (TODO: size).

- [Migrate off from github.com/gogo/protobuf](https://github.com/containerd/containerd/pull/6841)

## imgcrypt

containerd is depending on containerd/imgcrypt and imgcrypt is depending on containerd. In order to workaround the cyclic dependencies, I had to use Go's reflect package.

- [Use reflect to support diff.ApplyConfig with/without gogo's types.Any](https://github.com/containerd/imgcrypt/pull/75)
