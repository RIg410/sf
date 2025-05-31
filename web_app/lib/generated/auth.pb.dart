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

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'auth.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'auth.pbenum.dart';

class SendVerificationCodeResponse extends $pb.GeneratedMessage {
  factory SendVerificationCodeResponse({
    $core.int? leftTime,
    SendVerificationCodeError? error,
  }) {
    final result = create();
    if (leftTime != null) result.leftTime = leftTime;
    if (error != null) result.error = error;
    return result;
  }

  SendVerificationCodeResponse._();

  factory SendVerificationCodeResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory SendVerificationCodeResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SendVerificationCodeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'auth'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'leftTime', $pb.PbFieldType.O3)
    ..e<SendVerificationCodeError>(2, _omitFieldNames ? '' : 'error', $pb.PbFieldType.OE, defaultOrMaker: SendVerificationCodeError.INVALID_PHONE_NUMBER, valueOf: SendVerificationCodeError.valueOf, enumValues: SendVerificationCodeError.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendVerificationCodeResponse clone() => SendVerificationCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendVerificationCodeResponse copyWith(void Function(SendVerificationCodeResponse) updates) => super.copyWith((message) => updates(message as SendVerificationCodeResponse)) as SendVerificationCodeResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendVerificationCodeResponse create() => SendVerificationCodeResponse._();
  @$core.override
  SendVerificationCodeResponse createEmptyInstance() => create();
  static $pb.PbList<SendVerificationCodeResponse> createRepeated() => $pb.PbList<SendVerificationCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static SendVerificationCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendVerificationCodeResponse>(create);
  static SendVerificationCodeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get leftTime => $_getIZ(0);
  @$pb.TagNumber(1)
  set leftTime($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasLeftTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearLeftTime() => $_clearField(1);

  @$pb.TagNumber(2)
  SendVerificationCodeError get error => $_getN(1);
  @$pb.TagNumber(2)
  set error(SendVerificationCodeError value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasError() => $_has(1);
  @$pb.TagNumber(2)
  void clearError() => $_clearField(2);
}

class TgAuthResult extends $pb.GeneratedMessage {
  factory TgAuthResult({
    $core.String? token,
    TgAuthError? error,
  }) {
    final result = create();
    if (token != null) result.token = token;
    if (error != null) result.error = error;
    return result;
  }

  TgAuthResult._();

  factory TgAuthResult.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory TgAuthResult.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TgAuthResult', package: const $pb.PackageName(_omitMessageNames ? '' : 'auth'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..e<TgAuthError>(2, _omitFieldNames ? '' : 'error', $pb.PbFieldType.OE, defaultOrMaker: TgAuthError.INVALID_TOKEN, valueOf: TgAuthError.valueOf, enumValues: TgAuthError.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TgAuthResult clone() => TgAuthResult()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TgAuthResult copyWith(void Function(TgAuthResult) updates) => super.copyWith((message) => updates(message as TgAuthResult)) as TgAuthResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TgAuthResult create() => TgAuthResult._();
  @$core.override
  TgAuthResult createEmptyInstance() => create();
  static $pb.PbList<TgAuthResult> createRepeated() => $pb.PbList<TgAuthResult>();
  @$core.pragma('dart2js:noInline')
  static TgAuthResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TgAuthResult>(create);
  static TgAuthResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);

  @$pb.TagNumber(2)
  TgAuthError get error => $_getN(1);
  @$pb.TagNumber(2)
  set error(TgAuthError value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasError() => $_has(1);
  @$pb.TagNumber(2)
  void clearError() => $_clearField(2);
}

class VerifyCodeResponse extends $pb.GeneratedMessage {
  factory VerifyCodeResponse({
    $core.String? token,
    VerifyCodeError? error,
  }) {
    final result = create();
    if (token != null) result.token = token;
    if (error != null) result.error = error;
    return result;
  }

  VerifyCodeResponse._();

  factory VerifyCodeResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory VerifyCodeResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'VerifyCodeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'auth'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..e<VerifyCodeError>(2, _omitFieldNames ? '' : 'error', $pb.PbFieldType.OE, defaultOrMaker: VerifyCodeError.INVALID_PHONE, valueOf: VerifyCodeError.valueOf, enumValues: VerifyCodeError.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VerifyCodeResponse clone() => VerifyCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VerifyCodeResponse copyWith(void Function(VerifyCodeResponse) updates) => super.copyWith((message) => updates(message as VerifyCodeResponse)) as VerifyCodeResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static VerifyCodeResponse create() => VerifyCodeResponse._();
  @$core.override
  VerifyCodeResponse createEmptyInstance() => create();
  static $pb.PbList<VerifyCodeResponse> createRepeated() => $pb.PbList<VerifyCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static VerifyCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifyCodeResponse>(create);
  static VerifyCodeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);

  @$pb.TagNumber(2)
  VerifyCodeError get error => $_getN(1);
  @$pb.TagNumber(2)
  set error(VerifyCodeError value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasError() => $_has(1);
  @$pb.TagNumber(2)
  void clearError() => $_clearField(2);
}

class TgKeyRequest extends $pb.GeneratedMessage {
  factory TgKeyRequest({
    $core.String? key,
  }) {
    final result = create();
    if (key != null) result.key = key;
    return result;
  }

  TgKeyRequest._();

  factory TgKeyRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory TgKeyRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TgKeyRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'auth'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'key')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TgKeyRequest clone() => TgKeyRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TgKeyRequest copyWith(void Function(TgKeyRequest) updates) => super.copyWith((message) => updates(message as TgKeyRequest)) as TgKeyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TgKeyRequest create() => TgKeyRequest._();
  @$core.override
  TgKeyRequest createEmptyInstance() => create();
  static $pb.PbList<TgKeyRequest> createRepeated() => $pb.PbList<TgKeyRequest>();
  @$core.pragma('dart2js:noInline')
  static TgKeyRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TgKeyRequest>(create);
  static TgKeyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get key => $_getSZ(0);
  @$pb.TagNumber(1)
  set key($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => $_clearField(1);
}

class VerificationCodeRequest extends $pb.GeneratedMessage {
  factory VerificationCodeRequest({
    $core.String? phoneNumber,
  }) {
    final result = create();
    if (phoneNumber != null) result.phoneNumber = phoneNumber;
    return result;
  }

  VerificationCodeRequest._();

  factory VerificationCodeRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory VerificationCodeRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'VerificationCodeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'auth'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'phoneNumber')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VerificationCodeRequest clone() => VerificationCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VerificationCodeRequest copyWith(void Function(VerificationCodeRequest) updates) => super.copyWith((message) => updates(message as VerificationCodeRequest)) as VerificationCodeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static VerificationCodeRequest create() => VerificationCodeRequest._();
  @$core.override
  VerificationCodeRequest createEmptyInstance() => create();
  static $pb.PbList<VerificationCodeRequest> createRepeated() => $pb.PbList<VerificationCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static VerificationCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerificationCodeRequest>(create);
  static VerificationCodeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get phoneNumber => $_getSZ(0);
  @$pb.TagNumber(1)
  set phoneNumber($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPhoneNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearPhoneNumber() => $_clearField(1);
}

class VerifyCodeRequest extends $pb.GeneratedMessage {
  factory VerifyCodeRequest({
    $core.String? phoneNumber,
    $core.String? code,
  }) {
    final result = create();
    if (phoneNumber != null) result.phoneNumber = phoneNumber;
    if (code != null) result.code = code;
    return result;
  }

  VerifyCodeRequest._();

  factory VerifyCodeRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory VerifyCodeRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'VerifyCodeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'auth'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'phoneNumber')
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VerifyCodeRequest clone() => VerifyCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VerifyCodeRequest copyWith(void Function(VerifyCodeRequest) updates) => super.copyWith((message) => updates(message as VerifyCodeRequest)) as VerifyCodeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static VerifyCodeRequest create() => VerifyCodeRequest._();
  @$core.override
  VerifyCodeRequest createEmptyInstance() => create();
  static $pb.PbList<VerifyCodeRequest> createRepeated() => $pb.PbList<VerifyCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static VerifyCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifyCodeRequest>(create);
  static VerifyCodeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get phoneNumber => $_getSZ(0);
  @$pb.TagNumber(1)
  set phoneNumber($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPhoneNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearPhoneNumber() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => $_clearField(2);
}


const $core.bool _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
