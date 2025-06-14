//
//  Generated code. Do not modify.
//  source: locations.proto
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

@$core.Deprecated('Use locationViewDescriptor instead')
const LocationView$json = {
  '1': 'LocationView',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'address', '3': 3, '4': 1, '5': 9, '10': 'address'},
    {'1': 'working_hours', '3': 4, '4': 1, '5': 11, '6': '.locations.WorkingHoursView', '10': 'workingHours'},
    {'1': 'halls', '3': 5, '4': 3, '5': 11, '6': '.locations.HallView', '10': 'halls'},
    {'1': 'version', '3': 6, '4': 1, '5': 4, '10': 'version'},
  ],
};

/// Descriptor for `LocationView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List locationViewDescriptor = $convert.base64Decode(
    'CgxMb2NhdGlvblZpZXcSHAoCaWQYASABKAsyDC5pZC5PYmplY3RJZFICaWQSEgoEbmFtZRgCIA'
    'EoCVIEbmFtZRIYCgdhZGRyZXNzGAMgASgJUgdhZGRyZXNzEkAKDXdvcmtpbmdfaG91cnMYBCAB'
    'KAsyGy5sb2NhdGlvbnMuV29ya2luZ0hvdXJzVmlld1IMd29ya2luZ0hvdXJzEikKBWhhbGxzGA'
    'UgAygLMhMubG9jYXRpb25zLkhhbGxWaWV3UgVoYWxscxIYCgd2ZXJzaW9uGAYgASgEUgd2ZXJz'
    'aW9u');

@$core.Deprecated('Use workingHoursViewDescriptor instead')
const WorkingHoursView$json = {
  '1': 'WorkingHoursView',
  '2': [
    {'1': 'monday', '3': 1, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 0, '10': 'monday', '17': true},
    {'1': 'tuesday', '3': 2, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 1, '10': 'tuesday', '17': true},
    {'1': 'wednesday', '3': 3, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 2, '10': 'wednesday', '17': true},
    {'1': 'thursday', '3': 4, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 3, '10': 'thursday', '17': true},
    {'1': 'friday', '3': 5, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 4, '10': 'friday', '17': true},
    {'1': 'saturday', '3': 6, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 5, '10': 'saturday', '17': true},
    {'1': 'sunday', '3': 7, '4': 1, '5': 11, '6': '.locations.DayHoursView', '9': 6, '10': 'sunday', '17': true},
  ],
  '8': [
    {'1': '_monday'},
    {'1': '_tuesday'},
    {'1': '_wednesday'},
    {'1': '_thursday'},
    {'1': '_friday'},
    {'1': '_saturday'},
    {'1': '_sunday'},
  ],
};

/// Descriptor for `WorkingHoursView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workingHoursViewDescriptor = $convert.base64Decode(
    'ChBXb3JraW5nSG91cnNWaWV3EjQKBm1vbmRheRgBIAEoCzIXLmxvY2F0aW9ucy5EYXlIb3Vyc1'
    'ZpZXdIAFIGbW9uZGF5iAEBEjYKB3R1ZXNkYXkYAiABKAsyFy5sb2NhdGlvbnMuRGF5SG91cnNW'
    'aWV3SAFSB3R1ZXNkYXmIAQESOgoJd2VkbmVzZGF5GAMgASgLMhcubG9jYXRpb25zLkRheUhvdX'
    'JzVmlld0gCUgl3ZWRuZXNkYXmIAQESOAoIdGh1cnNkYXkYBCABKAsyFy5sb2NhdGlvbnMuRGF5'
    'SG91cnNWaWV3SANSCHRodXJzZGF5iAEBEjQKBmZyaWRheRgFIAEoCzIXLmxvY2F0aW9ucy5EYX'
    'lIb3Vyc1ZpZXdIBFIGZnJpZGF5iAEBEjgKCHNhdHVyZGF5GAYgASgLMhcubG9jYXRpb25zLkRh'
    'eUhvdXJzVmlld0gFUghzYXR1cmRheYgBARI0CgZzdW5kYXkYByABKAsyFy5sb2NhdGlvbnMuRG'
    'F5SG91cnNWaWV3SAZSBnN1bmRheYgBAUIJCgdfbW9uZGF5QgoKCF90dWVzZGF5QgwKCl93ZWRu'
    'ZXNkYXlCCwoJX3RodXJzZGF5QgkKB19mcmlkYXlCCwoJX3NhdHVyZGF5QgkKB19zdW5kYXk=');

@$core.Deprecated('Use dayHoursViewDescriptor instead')
const DayHoursView$json = {
  '1': 'DayHoursView',
  '2': [
    {'1': 'open', '3': 1, '4': 1, '5': 3, '10': 'open'},
    {'1': 'close', '3': 2, '4': 1, '5': 3, '10': 'close'},
  ],
};

/// Descriptor for `DayHoursView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dayHoursViewDescriptor = $convert.base64Decode(
    'CgxEYXlIb3Vyc1ZpZXcSEgoEb3BlbhgBIAEoA1IEb3BlbhIUCgVjbG9zZRgCIAEoA1IFY2xvc2'
    'U=');

@$core.Deprecated('Use hallViewDescriptor instead')
const HallView$json = {
  '1': 'HallView',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `HallView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List hallViewDescriptor = $convert.base64Decode(
    'CghIYWxsVmlldxIcCgJpZBgBIAEoCzIMLmlkLk9iamVjdElkUgJpZBISCgRuYW1lGAIgASgJUg'
    'RuYW1l');

@$core.Deprecated('Use locationRequestDescriptor instead')
const LocationRequest$json = {
  '1': 'LocationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
  ],
};

/// Descriptor for `LocationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List locationRequestDescriptor = $convert.base64Decode(
    'Cg9Mb2NhdGlvblJlcXVlc3QSHAoCaWQYASABKAsyDC5pZC5PYmplY3RJZFICaWQ=');

@$core.Deprecated('Use locationListRequestDescriptor instead')
const LocationListRequest$json = {
  '1': 'LocationListRequest',
};

/// Descriptor for `LocationListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List locationListRequestDescriptor = $convert.base64Decode(
    'ChNMb2NhdGlvbkxpc3RSZXF1ZXN0');

@$core.Deprecated('Use locationListViewDescriptor instead')
const LocationListView$json = {
  '1': 'LocationListView',
  '2': [
    {'1': 'locations', '3': 1, '4': 3, '5': 11, '6': '.locations.LocationView', '10': 'locations'},
  ],
};

/// Descriptor for `LocationListView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List locationListViewDescriptor = $convert.base64Decode(
    'ChBMb2NhdGlvbkxpc3RWaWV3EjUKCWxvY2F0aW9ucxgBIAMoCzIXLmxvY2F0aW9ucy5Mb2NhdG'
    'lvblZpZXdSCWxvY2F0aW9ucw==');

@$core.Deprecated('Use createLocationRequestDescriptor instead')
const CreateLocationRequest$json = {
  '1': 'CreateLocationRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'address', '3': 2, '4': 1, '5': 9, '10': 'address'},
    {'1': 'working_hours', '3': 3, '4': 1, '5': 11, '6': '.locations.WorkingHoursView', '10': 'workingHours'},
  ],
};

/// Descriptor for `CreateLocationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLocationRequestDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVMb2NhdGlvblJlcXVlc3QSEgoEbmFtZRgBIAEoCVIEbmFtZRIYCgdhZGRyZXNzGA'
    'IgASgJUgdhZGRyZXNzEkAKDXdvcmtpbmdfaG91cnMYAyABKAsyGy5sb2NhdGlvbnMuV29ya2lu'
    'Z0hvdXJzVmlld1IMd29ya2luZ0hvdXJz');

@$core.Deprecated('Use createLocationResponseDescriptor instead')
const CreateLocationResponse$json = {
  '1': 'CreateLocationResponse',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
  ],
};

/// Descriptor for `CreateLocationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLocationResponseDescriptor = $convert.base64Decode(
    'ChZDcmVhdGVMb2NhdGlvblJlc3BvbnNlEhwKAmlkGAEgASgLMgwuaWQuT2JqZWN0SWRSAmlk');

@$core.Deprecated('Use updateLocationRequestDescriptor instead')
const UpdateLocationRequest$json = {
  '1': 'UpdateLocationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'name', '17': true},
    {'1': 'address', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'address', '17': true},
    {'1': 'working_hours', '3': 4, '4': 1, '5': 11, '6': '.locations.WorkingHoursView', '9': 2, '10': 'workingHours', '17': true},
  ],
  '8': [
    {'1': '_name'},
    {'1': '_address'},
    {'1': '_working_hours'},
  ],
};

/// Descriptor for `UpdateLocationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateLocationRequestDescriptor = $convert.base64Decode(
    'ChVVcGRhdGVMb2NhdGlvblJlcXVlc3QSHAoCaWQYASABKAsyDC5pZC5PYmplY3RJZFICaWQSFw'
    'oEbmFtZRgCIAEoCUgAUgRuYW1liAEBEh0KB2FkZHJlc3MYAyABKAlIAVIHYWRkcmVzc4gBARJF'
    'Cg13b3JraW5nX2hvdXJzGAQgASgLMhsubG9jYXRpb25zLldvcmtpbmdIb3Vyc1ZpZXdIAlIMd2'
    '9ya2luZ0hvdXJziAEBQgcKBV9uYW1lQgoKCF9hZGRyZXNzQhAKDl93b3JraW5nX2hvdXJz');

@$core.Deprecated('Use updateLocationResponseDescriptor instead')
const UpdateLocationResponse$json = {
  '1': 'UpdateLocationResponse',
};

/// Descriptor for `UpdateLocationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateLocationResponseDescriptor = $convert.base64Decode(
    'ChZVcGRhdGVMb2NhdGlvblJlc3BvbnNl');

@$core.Deprecated('Use deleteLocationRequestDescriptor instead')
const DeleteLocationRequest$json = {
  '1': 'DeleteLocationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
  ],
};

/// Descriptor for `DeleteLocationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteLocationRequestDescriptor = $convert.base64Decode(
    'ChVEZWxldGVMb2NhdGlvblJlcXVlc3QSHAoCaWQYASABKAsyDC5pZC5PYmplY3RJZFICaWQ=');

@$core.Deprecated('Use deleteLocationResponseDescriptor instead')
const DeleteLocationResponse$json = {
  '1': 'DeleteLocationResponse',
};

/// Descriptor for `DeleteLocationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteLocationResponseDescriptor = $convert.base64Decode(
    'ChZEZWxldGVMb2NhdGlvblJlc3BvbnNl');

@$core.Deprecated('Use addHallRequestDescriptor instead')
const AddHallRequest$json = {
  '1': 'AddHallRequest',
  '2': [
    {'1': 'location_id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'locationId'},
    {'1': 'hall_name', '3': 2, '4': 1, '5': 9, '10': 'hallName'},
  ],
};

/// Descriptor for `AddHallRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addHallRequestDescriptor = $convert.base64Decode(
    'Cg5BZGRIYWxsUmVxdWVzdBItCgtsb2NhdGlvbl9pZBgBIAEoCzIMLmlkLk9iamVjdElkUgpsb2'
    'NhdGlvbklkEhsKCWhhbGxfbmFtZRgCIAEoCVIIaGFsbE5hbWU=');

@$core.Deprecated('Use addHallResponseDescriptor instead')
const AddHallResponse$json = {
  '1': 'AddHallResponse',
  '2': [
    {'1': 'hall_id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'hallId'},
  ],
};

/// Descriptor for `AddHallResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addHallResponseDescriptor = $convert.base64Decode(
    'Cg9BZGRIYWxsUmVzcG9uc2USJQoHaGFsbF9pZBgBIAEoCzIMLmlkLk9iamVjdElkUgZoYWxsSW'
    'Q=');

@$core.Deprecated('Use removeHallRequestDescriptor instead')
const RemoveHallRequest$json = {
  '1': 'RemoveHallRequest',
  '2': [
    {'1': 'location_id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'locationId'},
    {'1': 'hall_id', '3': 2, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'hallId'},
  ],
};

/// Descriptor for `RemoveHallRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeHallRequestDescriptor = $convert.base64Decode(
    'ChFSZW1vdmVIYWxsUmVxdWVzdBItCgtsb2NhdGlvbl9pZBgBIAEoCzIMLmlkLk9iamVjdElkUg'
    'psb2NhdGlvbklkEiUKB2hhbGxfaWQYAiABKAsyDC5pZC5PYmplY3RJZFIGaGFsbElk');

@$core.Deprecated('Use removeHallResponseDescriptor instead')
const RemoveHallResponse$json = {
  '1': 'RemoveHallResponse',
};

/// Descriptor for `RemoveHallResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeHallResponseDescriptor = $convert.base64Decode(
    'ChJSZW1vdmVIYWxsUmVzcG9uc2U=');

@$core.Deprecated('Use updateHallRequestDescriptor instead')
const UpdateHallRequest$json = {
  '1': 'UpdateHallRequest',
  '2': [
    {'1': 'location_id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'locationId'},
    {'1': 'hall_id', '3': 2, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'hallId'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `UpdateHallRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateHallRequestDescriptor = $convert.base64Decode(
    'ChFVcGRhdGVIYWxsUmVxdWVzdBItCgtsb2NhdGlvbl9pZBgBIAEoCzIMLmlkLk9iamVjdElkUg'
    'psb2NhdGlvbklkEiUKB2hhbGxfaWQYAiABKAsyDC5pZC5PYmplY3RJZFIGaGFsbElkEhIKBG5h'
    'bWUYAyABKAlSBG5hbWU=');

@$core.Deprecated('Use updateHallResponseDescriptor instead')
const UpdateHallResponse$json = {
  '1': 'UpdateHallResponse',
};

/// Descriptor for `UpdateHallResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateHallResponseDescriptor = $convert.base64Decode(
    'ChJVcGRhdGVIYWxsUmVzcG9uc2U=');

