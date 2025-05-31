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

class VerifyCodeError extends $pb.ProtobufEnum {
  static const VerifyCodeError INVALID_PHONE = VerifyCodeError._(0, _omitEnumNames ? '' : 'INVALID_PHONE');
  static const VerifyCodeError INVALID_CODE = VerifyCodeError._(1, _omitEnumNames ? '' : 'INVALID_CODE');
  static const VerifyCodeError EXPIRED = VerifyCodeError._(2, _omitEnumNames ? '' : 'EXPIRED');
  static const VerifyCodeError TOO_MANY_ATTEMPTS = VerifyCodeError._(3, _omitEnumNames ? '' : 'TOO_MANY_ATTEMPTS');

  static const $core.List<VerifyCodeError> values = <VerifyCodeError> [
    INVALID_PHONE,
    INVALID_CODE,
    EXPIRED,
    TOO_MANY_ATTEMPTS,
  ];

  static final $core.List<VerifyCodeError?> _byValue = $pb.ProtobufEnum.$_initByValueList(values, 3);
  static VerifyCodeError? valueOf($core.int value) =>  value < 0 || value >= _byValue.length ? null : _byValue[value];

  const VerifyCodeError._(super.value, super.name);
}

class TgAuthError extends $pb.ProtobufEnum {
  static const TgAuthError INVALID_TOKEN = TgAuthError._(0, _omitEnumNames ? '' : 'INVALID_TOKEN');
  static const TgAuthError TOO_OLD_TOKEN = TgAuthError._(1, _omitEnumNames ? '' : 'TOO_OLD_TOKEN');
  static const TgAuthError USER_NOT_FOUND = TgAuthError._(2, _omitEnumNames ? '' : 'USER_NOT_FOUND');

  static const $core.List<TgAuthError> values = <TgAuthError> [
    INVALID_TOKEN,
    TOO_OLD_TOKEN,
    USER_NOT_FOUND,
  ];

  static final $core.List<TgAuthError?> _byValue = $pb.ProtobufEnum.$_initByValueList(values, 2);
  static TgAuthError? valueOf($core.int value) =>  value < 0 || value >= _byValue.length ? null : _byValue[value];

  const TgAuthError._(super.value, super.name);
}

class SendVerificationCodeError extends $pb.ProtobufEnum {
  static const SendVerificationCodeError INVALID_PHONE_NUMBER = SendVerificationCodeError._(0, _omitEnumNames ? '' : 'INVALID_PHONE_NUMBER');
  static const SendVerificationCodeError V_USER_NOT_FOUND = SendVerificationCodeError._(1, _omitEnumNames ? '' : 'V_USER_NOT_FOUND');
  static const SendVerificationCodeError ALREADY_SENT = SendVerificationCodeError._(2, _omitEnumNames ? '' : 'ALREADY_SENT');
  static const SendVerificationCodeError NOT_AVAILABLE = SendVerificationCodeError._(3, _omitEnumNames ? '' : 'NOT_AVAILABLE');

  static const $core.List<SendVerificationCodeError> values = <SendVerificationCodeError> [
    INVALID_PHONE_NUMBER,
    V_USER_NOT_FOUND,
    ALREADY_SENT,
    NOT_AVAILABLE,
  ];

  static final $core.List<SendVerificationCodeError?> _byValue = $pb.ProtobufEnum.$_initByValueList(values, 3);
  static SendVerificationCodeError? valueOf($core.int value) =>  value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SendVerificationCodeError._(super.value, super.name);
}


const $core.bool _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
