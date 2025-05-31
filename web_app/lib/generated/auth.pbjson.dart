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

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use verifyCodeErrorDescriptor instead')
const VerifyCodeError$json = {
  '1': 'VerifyCodeError',
  '2': [
    {'1': 'INVALID_PHONE', '2': 0},
    {'1': 'INVALID_CODE', '2': 1},
    {'1': 'EXPIRED', '2': 2},
    {'1': 'TOO_MANY_ATTEMPTS', '2': 3},
  ],
};

/// Descriptor for `VerifyCodeError`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List verifyCodeErrorDescriptor = $convert.base64Decode(
    'Cg9WZXJpZnlDb2RlRXJyb3ISEQoNSU5WQUxJRF9QSE9ORRAAEhAKDElOVkFMSURfQ09ERRABEg'
    'sKB0VYUElSRUQQAhIVChFUT09fTUFOWV9BVFRFTVBUUxAD');

@$core.Deprecated('Use tgAuthErrorDescriptor instead')
const TgAuthError$json = {
  '1': 'TgAuthError',
  '2': [
    {'1': 'INVALID_TOKEN', '2': 0},
    {'1': 'TOO_OLD_TOKEN', '2': 1},
    {'1': 'USER_NOT_FOUND', '2': 2},
  ],
};

/// Descriptor for `TgAuthError`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List tgAuthErrorDescriptor = $convert.base64Decode(
    'CgtUZ0F1dGhFcnJvchIRCg1JTlZBTElEX1RPS0VOEAASEQoNVE9PX09MRF9UT0tFThABEhIKDl'
    'VTRVJfTk9UX0ZPVU5EEAI=');

@$core.Deprecated('Use sendVerificationCodeErrorDescriptor instead')
const SendVerificationCodeError$json = {
  '1': 'SendVerificationCodeError',
  '2': [
    {'1': 'INVALID_PHONE_NUMBER', '2': 0},
    {'1': 'V_USER_NOT_FOUND', '2': 1},
    {'1': 'ALREADY_SENT', '2': 2},
    {'1': 'NOT_AVAILABLE', '2': 3},
  ],
};

/// Descriptor for `SendVerificationCodeError`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List sendVerificationCodeErrorDescriptor = $convert.base64Decode(
    'ChlTZW5kVmVyaWZpY2F0aW9uQ29kZUVycm9yEhgKFElOVkFMSURfUEhPTkVfTlVNQkVSEAASFA'
    'oQVl9VU0VSX05PVF9GT1VORBABEhAKDEFMUkVBRFlfU0VOVBACEhEKDU5PVF9BVkFJTEFCTEUQ'
    'Aw==');

@$core.Deprecated('Use sendVerificationCodeResponseDescriptor instead')
const SendVerificationCodeResponse$json = {
  '1': 'SendVerificationCodeResponse',
  '2': [
    {'1': 'left_time', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'leftTime', '17': true},
    {'1': 'error', '3': 2, '4': 1, '5': 14, '6': '.auth.SendVerificationCodeError', '9': 1, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_left_time'},
    {'1': '_error'},
  ],
};

/// Descriptor for `SendVerificationCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendVerificationCodeResponseDescriptor = $convert.base64Decode(
    'ChxTZW5kVmVyaWZpY2F0aW9uQ29kZVJlc3BvbnNlEiAKCWxlZnRfdGltZRgBIAEoBUgAUghsZW'
    'Z0VGltZYgBARI6CgVlcnJvchgCIAEoDjIfLmF1dGguU2VuZFZlcmlmaWNhdGlvbkNvZGVFcnJv'
    'ckgBUgVlcnJvcogBAUIMCgpfbGVmdF90aW1lQggKBl9lcnJvcg==');

@$core.Deprecated('Use tgAuthResultDescriptor instead')
const TgAuthResult$json = {
  '1': 'TgAuthResult',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'token', '17': true},
    {'1': 'error', '3': 2, '4': 1, '5': 14, '6': '.auth.TgAuthError', '9': 1, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_token'},
    {'1': '_error'},
  ],
};

/// Descriptor for `TgAuthResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List tgAuthResultDescriptor = $convert.base64Decode(
    'CgxUZ0F1dGhSZXN1bHQSGQoFdG9rZW4YASABKAlIAFIFdG9rZW6IAQESLAoFZXJyb3IYAiABKA'
    '4yES5hdXRoLlRnQXV0aEVycm9ySAFSBWVycm9yiAEBQggKBl90b2tlbkIICgZfZXJyb3I=');

@$core.Deprecated('Use verifyCodeResponseDescriptor instead')
const VerifyCodeResponse$json = {
  '1': 'VerifyCodeResponse',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'token', '17': true},
    {'1': 'error', '3': 2, '4': 1, '5': 14, '6': '.auth.VerifyCodeError', '9': 1, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_token'},
    {'1': '_error'},
  ],
};

/// Descriptor for `VerifyCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyCodeResponseDescriptor = $convert.base64Decode(
    'ChJWZXJpZnlDb2RlUmVzcG9uc2USGQoFdG9rZW4YASABKAlIAFIFdG9rZW6IAQESMAoFZXJyb3'
    'IYAiABKA4yFS5hdXRoLlZlcmlmeUNvZGVFcnJvckgBUgVlcnJvcogBAUIICgZfdG9rZW5CCAoG'
    'X2Vycm9y');

@$core.Deprecated('Use tgKeyRequestDescriptor instead')
const TgKeyRequest$json = {
  '1': 'TgKeyRequest',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
  ],
};

/// Descriptor for `TgKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List tgKeyRequestDescriptor = $convert.base64Decode(
    'CgxUZ0tleVJlcXVlc3QSEAoDa2V5GAEgASgJUgNrZXk=');

@$core.Deprecated('Use verificationCodeRequestDescriptor instead')
const VerificationCodeRequest$json = {
  '1': 'VerificationCodeRequest',
  '2': [
    {'1': 'phone_number', '3': 1, '4': 1, '5': 9, '10': 'phoneNumber'},
  ],
};

/// Descriptor for `VerificationCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verificationCodeRequestDescriptor = $convert.base64Decode(
    'ChdWZXJpZmljYXRpb25Db2RlUmVxdWVzdBIhCgxwaG9uZV9udW1iZXIYASABKAlSC3Bob25lTn'
    'VtYmVy');

@$core.Deprecated('Use verifyCodeRequestDescriptor instead')
const VerifyCodeRequest$json = {
  '1': 'VerifyCodeRequest',
  '2': [
    {'1': 'phone_number', '3': 1, '4': 1, '5': 9, '10': 'phoneNumber'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
  ],
};

/// Descriptor for `VerifyCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyCodeRequestDescriptor = $convert.base64Decode(
    'ChFWZXJpZnlDb2RlUmVxdWVzdBIhCgxwaG9uZV9udW1iZXIYASABKAlSC3Bob25lTnVtYmVyEh'
    'IKBGNvZGUYAiABKAlSBGNvZGU=');

