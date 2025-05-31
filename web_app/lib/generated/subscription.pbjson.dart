//
//  Generated code. Do not modify.
//  source: subscription.proto
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

@$core.Deprecated('Use subscriptionTypeViewDescriptor instead')
const SubscriptionTypeView$json = {
  '1': 'SubscriptionTypeView',
  '2': [
    {'1': 'group', '3': 1, '4': 1, '5': 11, '6': '.subscription.Group', '9': 0, '10': 'group'},
    {'1': 'personal', '3': 2, '4': 1, '5': 11, '6': '.subscription.Personal', '9': 0, '10': 'personal'},
  ],
  '8': [
    {'1': 'subscription_type'},
  ],
};

/// Descriptor for `SubscriptionTypeView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List subscriptionTypeViewDescriptor = $convert.base64Decode(
    'ChRTdWJzY3JpcHRpb25UeXBlVmlldxIrCgVncm91cBgBIAEoCzITLnN1YnNjcmlwdGlvbi5Hcm'
    '91cEgAUgVncm91cBI0CghwZXJzb25hbBgCIAEoCzIWLnN1YnNjcmlwdGlvbi5QZXJzb25hbEgA'
    'UghwZXJzb25hbEITChFzdWJzY3JpcHRpb25fdHlwZQ==');

@$core.Deprecated('Use groupDescriptor instead')
const Group$json = {
  '1': 'Group',
  '2': [
    {'1': 'program_filter', '3': 1, '4': 3, '5': 11, '6': '.id.ObjectId', '10': 'programFilter'},
  ],
};

/// Descriptor for `Group`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupDescriptor = $convert.base64Decode(
    'CgVHcm91cBIzCg5wcm9ncmFtX2ZpbHRlchgBIAMoCzIMLmlkLk9iamVjdElkUg1wcm9ncmFtRm'
    'lsdGVy');

@$core.Deprecated('Use personalDescriptor instead')
const Personal$json = {
  '1': 'Personal',
  '2': [
    {'1': 'couch_filter', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'couchFilter'},
  ],
};

/// Descriptor for `Personal`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personalDescriptor = $convert.base64Decode(
    'CghQZXJzb25hbBIvCgxjb3VjaF9maWx0ZXIYASABKAsyDC5pZC5PYmplY3RJZFILY291Y2hGaW'
    'x0ZXI=');

