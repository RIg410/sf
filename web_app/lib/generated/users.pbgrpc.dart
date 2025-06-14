//
//  Generated code. Do not modify.
//  source: users.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'user.pb.dart' as $3;
import 'users.pb.dart' as $2;

export 'users.pb.dart';

@$pb.GrpcServiceName('users.UsersService')
class UsersServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  static final _$get = $grpc.ClientMethod<$2.UserRequest, $3.UserView>(
      '/users.UsersService/get',
      ($2.UserRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.UserView.fromBuffer(value));

  UsersServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$3.UserView> get($2.UserRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }
}

@$pb.GrpcServiceName('users.UsersService')
abstract class UsersServiceBase extends $grpc.Service {
  $core.String get $name => 'users.UsersService';

  UsersServiceBase() {
    $addMethod($grpc.ServiceMethod<$2.UserRequest, $3.UserView>(
        'get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $2.UserRequest.fromBuffer(value),
        ($3.UserView value) => value.writeToBuffer()));
  }

  $async.Future<$3.UserView> get_Pre($grpc.ServiceCall $call, $async.Future<$2.UserRequest> $request) async {
    return get($call, await $request);
  }

  $async.Future<$3.UserView> get($grpc.ServiceCall call, $2.UserRequest request);
}
