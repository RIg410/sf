#!/bin/bash

mkdir -p lib/generated

protoc  --proto_path=../api/proto --dart_out=grpc:lib/generated  ../api/proto/*.proto

