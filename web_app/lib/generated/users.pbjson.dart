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

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use userRequestDescriptor instead')
const UserRequest$json = {
  '1': 'UserRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '9': 0, '10': 'id', '17': true},
  ],
  '8': [
    {'1': '_id'},
  ],
};

/// Descriptor for `UserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userRequestDescriptor = $convert.base64Decode(
    'CgtVc2VyUmVxdWVzdBIhCgJpZBgBIAEoCzIMLmlkLk9iamVjdElkSABSAmlkiAEBQgUKA19pZA'
    '==');

@$core.Deprecated('Use userFilterDescriptor instead')
const UserFilter$json = {
  '1': 'UserFilter',
};

/// Descriptor for `UserFilter`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userFilterDescriptor = $convert.base64Decode(
    'CgpVc2VyRmlsdGVy');

