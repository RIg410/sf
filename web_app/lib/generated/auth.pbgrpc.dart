//
//  Generated code. Do not modify.
//  source: auth.proto
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

import 'auth.pb.dart' as $0;

export 'auth.pb.dart';

@$pb.GrpcServiceName('auth.AuthService')
class AuthServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  static final _$tg_auth = $grpc.ClientMethod<$0.TgKeyRequest, $0.TgAuthResult>(
      '/auth.AuthService/tg_auth',
      ($0.TgKeyRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.TgAuthResult.fromBuffer(value));
  static final _$send_verification_code = $grpc.ClientMethod<$0.VerificationCodeRequest, $0.SendVerificationCodeResponse>(
      '/auth.AuthService/send_verification_code',
      ($0.VerificationCodeRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.SendVerificationCodeResponse.fromBuffer(value));
  static final _$verify_code = $grpc.ClientMethod<$0.VerifyCodeRequest, $0.VerifyCodeResponse>(
      '/auth.AuthService/verify_code',
      ($0.VerifyCodeRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.VerifyCodeResponse.fromBuffer(value));

  AuthServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.TgAuthResult> tg_auth($0.TgKeyRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$tg_auth, request, options: options);
  }

  $grpc.ResponseFuture<$0.SendVerificationCodeResponse> send_verification_code($0.VerificationCodeRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$send_verification_code, request, options: options);
  }

  $grpc.ResponseFuture<$0.VerifyCodeResponse> verify_code($0.VerifyCodeRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$verify_code, request, options: options);
  }
}

@$pb.GrpcServiceName('auth.AuthService')
abstract class AuthServiceBase extends $grpc.Service {
  $core.String get $name => 'auth.AuthService';

  AuthServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.TgKeyRequest, $0.TgAuthResult>(
        'tg_auth',
        tg_auth_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.TgKeyRequest.fromBuffer(value),
        ($0.TgAuthResult value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.VerificationCodeRequest, $0.SendVerificationCodeResponse>(
        'send_verification_code',
        send_verification_code_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.VerificationCodeRequest.fromBuffer(value),
        ($0.SendVerificationCodeResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.VerifyCodeRequest, $0.VerifyCodeResponse>(
        'verify_code',
        verify_code_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.VerifyCodeRequest.fromBuffer(value),
        ($0.VerifyCodeResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.TgAuthResult> tg_auth_Pre($grpc.ServiceCall $call, $async.Future<$0.TgKeyRequest> $request) async {
    return tg_auth($call, await $request);
  }

  $async.Future<$0.SendVerificationCodeResponse> send_verification_code_Pre($grpc.ServiceCall $call, $async.Future<$0.VerificationCodeRequest> $request) async {
    return send_verification_code($call, await $request);
  }

  $async.Future<$0.VerifyCodeResponse> verify_code_Pre($grpc.ServiceCall $call, $async.Future<$0.VerifyCodeRequest> $request) async {
    return verify_code($call, await $request);
  }

  $async.Future<$0.TgAuthResult> tg_auth($grpc.ServiceCall call, $0.TgKeyRequest request);
  $async.Future<$0.SendVerificationCodeResponse> send_verification_code($grpc.ServiceCall call, $0.VerificationCodeRequest request);
  $async.Future<$0.VerifyCodeResponse> verify_code($grpc.ServiceCall call, $0.VerifyCodeRequest request);
}
