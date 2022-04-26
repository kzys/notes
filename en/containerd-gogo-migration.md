# containerd's gogo/protobuf migration

containerd has migrated from
[github.com/gogo/protobuf](https://github.com/gogo/protobuf/) to
[google.golang.org/protobuf](https://github.com/protocolbuffers/protobuf-go) in April 2022.
This document describes the effort.

## Background

containerd has been using gogo/protobuf since the very beginning (TODO: when?). While I wasn't involving the original decision, using gogo/protobuf seemed popular at that time. For example, the following project are/were using gogo/protobuf;

- [Vitess](https://github.com/vitessio/vitess) ([migrating off in 2021](https://vitess.io/blog/2021-06-03-a-new-protobuf-generator-for-go/))
- [Istio](https://github.com/istio/istio) ([migrating off in 2022](https://github.com/istio/istio/pull/38055))
- [etcd](https://github.com/etcd-io/etcd)
- [CockroachDB](https://github.com/cockroachdb/cockroach)

Compared to Google's original Go Protocol Buffers package, gogo/protobuf was much performant and allowed more customization regarding code generation.

However, the maintainers of gogo/protobuf decided to step down and the project is [looking for new maintainers since May 2020](https://github.com/gogo/protobuf/issues/691).

> Unfortunately, the personal circumstances of the maintainers have changed and we are no longer able to keep up with the issues and feature requests that naturally crop up for a popular open source project.
>
> In particular, the recent golang/protobuf release 1.4.x (AKA APIv2: <https://blog.golang.org/protobuf-apiv2>), has created a big chunk of work required to be compatible with the new world of Go protobufs.

While taking the ownership was technically possible, maintaining the RPC package was not really containerd's focus. So I started the migration effort in 2021 (TODO: links).

## containerd/protobuild

containerd is using [Protobuild](https://github.com/containerd/protobuild) that wraps protoc-gen-go and/or other code generators that take `.proto` files. Previously protoc-gen-go was supporting gRPC through its "plugin" mechanism, but the design has been changed since (TODO: version). Now protoc-gen-go is only responsible for Protocol Buffers, and gRPC needs protoc-gen-go-grpc.

Supporting multiple generators was lacking in Protobuild. I extended Protobuild to support protoc-gen-go and protoc-gen-go-grpc.

- [Support multiple generators](https://github.com/containerd/protobuild/pull/45)
- [Remove plugins and import_path](https://github.com/containerd/protobuild/pull/48)

In addition to that, protoc-gen-go doesn't allow much customization compared to gogo/protobuf. I had to write a small utility program that rewrites Go's AST.

- [Add go-fix-acronym](https://github.com/containerd/protobuild/pull/50)

## containerd/ttrpc

ttrpc is containerd's own RPC protocol, which is basically "gRPC without HTTP".

I have removed gogo/protobuf from ttrpc, and written a new code generator that could be used with protoc-gen-go.

- [Use google.golang.org/protobuf instead of github.com/gogo/protobuf](https://github.com/containerd/ttrpc/pull/99)
- [Add protoc-gen-go-ttrpc](https://github.com/containerd/ttrpc/pull/96)

## containerd/containerd

containerd has been using a lot of gogo/protobuf extensions. I have removed all of them in the following PRs. Luckily containerd was converting these auto-generated structs to internal domain models right after getting the messages. The blast radius of the changes was much smaller than I initially expected.

- [Remove gogoproto.customtype](https://github.com/containerd/containerd/pull/6699)
- [Remove enumvalue_customname, goproto_enum_prefix and enum_customname](https://github.com/containerd/containerd/pull/6708)
- [Remove gogoproto.stdtime](https://github.com/containerd/containerd/pull/6821)
- [Remove all gogoproto extensions](https://github.com/containerd/containerd/pull/6829)
- [Consolidate gogo/protobuf dependencies under our own protobuf package](https://github.com/containerd/containerd/pull/6826)

Because of the PRs above, the size of the final migration PR was manageable (TODO: size).

- [Migrate off from github.com/gogo/protobuf](https://github.com/containerd/containerd/pull/6841)

## containerd/imgcrypt

containerd is depending on containerd/imgcrypt and imgcrypt is depending on containerd. In order to workaround the cyclic dependencies, I had to use Go's reflect package.

- [Use reflect to support diff.ApplyConfig with/without gogo's types.Any](https://github.com/containerd/imgcrypt/pull/75)

## Retrospective

### Went Well

- Submitted small incremental PRs instead of having a big one.

### Needs Improvements

- It wasn't really estimated and I haven't discussed much about the work inside Amazon.
- Possibly due to that, I wasn't asking much help except for code reviews, inside/outside Amazon.
