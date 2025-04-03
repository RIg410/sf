#!/bin/bash
protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_opt=outputClientImpl=grpc-web --ts_proto_out=./src/generated --proto_path ../proto  ../proto/auth.proto