//
//  Generated code. Do not modify.
//  source: user.proto
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

@$core.Deprecated('Use sourceViewDescriptor instead')
const SourceView$json = {
  '1': 'SourceView',
  '2': [
    {'1': 'UNKNOWN', '2': 0},
    {'1': 'WEBSITE', '2': 1},
    {'1': 'INSTAGRAM', '2': 2},
    {'1': 'VK', '2': 3},
    {'1': 'YANDEX_MAP', '2': 4},
    {'1': 'YANDEX_DIRECT', '2': 5},
    {'1': 'DIRECT_ADDS', '2': 6},
    {'1': 'VK_ADDS', '2': 7},
    {'1': 'DOUBLE_GIS', '2': 8},
    {'1': 'AVITO', '2': 9},
    {'1': 'RECOMMENDATION', '2': 10},
    {'1': 'OTHER', '2': 11},
    {'1': 'WEB_SEARCH', '2': 12},
    {'1': 'OLD_BASE', '2': 13},
  ],
};

/// Descriptor for `SourceView`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List sourceViewDescriptor = $convert.base64Decode(
    'CgpTb3VyY2VWaWV3EgsKB1VOS05PV04QABILCgdXRUJTSVRFEAESDQoJSU5TVEFHUkFNEAISBg'
    'oCVksQAxIOCgpZQU5ERVhfTUFQEAQSEQoNWUFOREVYX0RJUkVDVBAFEg8KC0RJUkVDVF9BRERT'
    'EAYSCwoHVktfQUREUxAHEg4KCkRPVUJMRV9HSVMQCBIJCgVBVklUTxAJEhIKDlJFQ09NTUVORE'
    'FUSU9OEAoSCQoFT1RIRVIQCxIOCgpXRUJfU0VBUkNIEAwSDAoIT0xEX0JBU0UQDQ==');

@$core.Deprecated('Use ruleViewDescriptor instead')
const RuleView$json = {
  '1': 'RuleView',
  '2': [
    {'1': 'VIEW_PROFILE', '2': 0},
    {'1': 'VIEW_USERS', '2': 1},
    {'1': 'EDIT_USER_RIGHTS', '2': 2},
    {'1': 'BLOCK_USER', '2': 3},
    {'1': 'EDIT_USER_INFO', '2': 4},
    {'1': 'EDIT_USER_SUBSCRIPTION', '2': 5},
    {'1': 'FREEZE_USERS', '2': 6},
    {'1': 'CHANGE_BALANCE', '2': 7},
    {'1': 'EDIT_MARKETING_INFO', '2': 8},
    {'1': 'EDIT_FAMILY', '2': 9},
    {'1': 'VIEW_FAMILY', '2': 10},
    {'1': 'EDIT_AI_PROMPT', '2': 11},
    {'1': 'VIEW_USER_COMMENTS', '2': 12},
    {'1': 'EDIT_USER_COMMENTS', '2': 13},
    {'1': 'DELETE_USER_COMMENTS', '2': 14},
    {'1': 'EDIT_TRAINING', '2': 15},
    {'1': 'CREATE_TRAINING', '2': 16},
    {'1': 'EDIT_TRAINING_CLIENTS_LIST', '2': 17},
    {'1': 'SET_KEEP_OPEN', '2': 18},
    {'1': 'SET_FREE', '2': 19},
    {'1': 'EDIT_SCHEDULE', '2': 20},
    {'1': 'CANCEL_TRAINING', '2': 21},
    {'1': 'REMOVE_TRAINING', '2': 22},
    {'1': 'EDIT_TRAINING_COUCH', '2': 23},
    {'1': 'SCHEDULE_GROUP_TRAINING', '2': 24},
    {'1': 'SCHEDULE_PERSONAL_TRAINING', '2': 25},
    {'1': 'SCHEDULE_SUB_RENT', '2': 26},
    {'1': 'SELECT_PERSONAL_INSTRUCTOR', '2': 27},
    {'1': 'VIEW_ALL_TRAININGS', '2': 28},
    {'1': 'CHANGE_TRAINING_SLOT', '2': 29},
    {'1': 'CREATE_SUBSCRIPTION', '2': 30},
    {'1': 'EDIT_SUBSCRIPTION', '2': 31},
    {'1': 'SELL_SUBSCRIPTION', '2': 32},
    {'1': 'FREE_SELL', '2': 33},
    {'1': 'SUB_RENT', '2': 34},
    {'1': 'VIEW_FINANCE', '2': 35},
    {'1': 'MAKE_PAYMENT', '2': 36},
    {'1': 'MAKE_DEPOSIT', '2': 37},
    {'1': 'FINANCE_HISTORICAL_DATE', '2': 38},
    {'1': 'DELETE_HISTORY', '2': 39},
    {'1': 'VIEW_EMPLOYEES', '2': 40},
    {'1': 'EDIT_EMPLOYEE', '2': 41},
    {'1': 'EDIT_EMPLOYEE_RATES', '2': 42},
    {'1': 'VIEW_LOGS', '2': 43},
    {'1': 'CREATE_COUCH', '2': 44},
    {'1': 'EDIT_COUCH', '2': 45},
    {'1': 'VIEW_COUCH_RATES', '2': 46},
    {'1': 'VIEW_STATISTICS', '2': 47},
    {'1': 'SYSTEM', '2': 48},
    {'1': 'VIEW_REWARDS', '2': 49},
    {'1': 'RECALCULATE_REWARDS', '2': 50},
    {'1': 'VIEW_MARKETING_INFO', '2': 51},
    {'1': 'CREATE_REQUEST', '2': 52},
    {'1': 'REQUESTS_HISTORY', '2': 53},
    {'1': 'RECEIVE_NOTIFICATIONS_ABOUT_SUBSCRIPTIONS', '2': 54},
    {'1': 'RECEIVE_NOTIFICATIONS_ABOUT_BIRTHDAYS', '2': 55},
    {'1': 'RECEIVE_AI_NOTIFICATIONS', '2': 56},
    {'1': 'MINI_APP', '2': 57},
    {'1': 'BUY_SUBSCRIPTION', '2': 58},
    {'1': 'VIEW_HIDDEN_PROGRAMS', '2': 59},
    {'1': 'HISTORY_VIEWER', '2': 60},
    {'1': 'AI_STATISTIC', '2': 61},
    {'1': 'AI_USER_INFO', '2': 62},
    {'1': 'SELECT_MODEL', '2': 63},
  ],
};

/// Descriptor for `RuleView`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List ruleViewDescriptor = $convert.base64Decode(
    'CghSdWxlVmlldxIQCgxWSUVXX1BST0ZJTEUQABIOCgpWSUVXX1VTRVJTEAESFAoQRURJVF9VU0'
    'VSX1JJR0hUUxACEg4KCkJMT0NLX1VTRVIQAxISCg5FRElUX1VTRVJfSU5GTxAEEhoKFkVESVRf'
    'VVNFUl9TVUJTQ1JJUFRJT04QBRIQCgxGUkVFWkVfVVNFUlMQBhISCg5DSEFOR0VfQkFMQU5DRR'
    'AHEhcKE0VESVRfTUFSS0VUSU5HX0lORk8QCBIPCgtFRElUX0ZBTUlMWRAJEg8KC1ZJRVdfRkFN'
    'SUxZEAoSEgoORURJVF9BSV9QUk9NUFQQCxIWChJWSUVXX1VTRVJfQ09NTUVOVFMQDBIWChJFRE'
    'lUX1VTRVJfQ09NTUVOVFMQDRIYChRERUxFVEVfVVNFUl9DT01NRU5UUxAOEhEKDUVESVRfVFJB'
    'SU5JTkcQDxITCg9DUkVBVEVfVFJBSU5JTkcQEBIeChpFRElUX1RSQUlOSU5HX0NMSUVOVFNfTE'
    'lTVBAREhEKDVNFVF9LRUVQX09QRU4QEhIMCghTRVRfRlJFRRATEhEKDUVESVRfU0NIRURVTEUQ'
    'FBITCg9DQU5DRUxfVFJBSU5JTkcQFRITCg9SRU1PVkVfVFJBSU5JTkcQFhIXChNFRElUX1RSQU'
    'lOSU5HX0NPVUNIEBcSGwoXU0NIRURVTEVfR1JPVVBfVFJBSU5JTkcQGBIeChpTQ0hFRFVMRV9Q'
    'RVJTT05BTF9UUkFJTklORxAZEhUKEVNDSEVEVUxFX1NVQl9SRU5UEBoSHgoaU0VMRUNUX1BFUl'
    'NPTkFMX0lOU1RSVUNUT1IQGxIWChJWSUVXX0FMTF9UUkFJTklOR1MQHBIYChRDSEFOR0VfVFJB'
    'SU5JTkdfU0xPVBAdEhcKE0NSRUFURV9TVUJTQ1JJUFRJT04QHhIVChFFRElUX1NVQlNDUklQVE'
    'lPThAfEhUKEVNFTExfU1VCU0NSSVBUSU9OECASDQoJRlJFRV9TRUxMECESDAoIU1VCX1JFTlQQ'
    'IhIQCgxWSUVXX0ZJTkFOQ0UQIxIQCgxNQUtFX1BBWU1FTlQQJBIQCgxNQUtFX0RFUE9TSVQQJR'
    'IbChdGSU5BTkNFX0hJU1RPUklDQUxfREFURRAmEhIKDkRFTEVURV9ISVNUT1JZECcSEgoOVklF'
    'V19FTVBMT1lFRVMQKBIRCg1FRElUX0VNUExPWUVFECkSFwoTRURJVF9FTVBMT1lFRV9SQVRFUx'
    'AqEg0KCVZJRVdfTE9HUxArEhAKDENSRUFURV9DT1VDSBAsEg4KCkVESVRfQ09VQ0gQLRIUChBW'
    'SUVXX0NPVUNIX1JBVEVTEC4SEwoPVklFV19TVEFUSVNUSUNTEC8SCgoGU1lTVEVNEDASEAoMVk'
    'lFV19SRVdBUkRTEDESFwoTUkVDQUxDVUxBVEVfUkVXQVJEUxAyEhcKE1ZJRVdfTUFSS0VUSU5H'
    'X0lORk8QMxISCg5DUkVBVEVfUkVRVUVTVBA0EhQKEFJFUVVFU1RTX0hJU1RPUlkQNRItCilSRU'
    'NFSVZFX05PVElGSUNBVElPTlNfQUJPVVRfU1VCU0NSSVBUSU9OUxA2EikKJVJFQ0VJVkVfTk9U'
    'SUZJQ0FUSU9OU19BQk9VVF9CSVJUSERBWVMQNxIcChhSRUNFSVZFX0FJX05PVElGSUNBVElPTl'
    'MQOBIMCghNSU5JX0FQUBA5EhQKEEJVWV9TVUJTQ1JJUFRJT04QOhIYChRWSUVXX0hJRERFTl9Q'
    'Uk9HUkFNUxA7EhIKDkhJU1RPUllfVklFV0VSEDwSEAoMQUlfU1RBVElTVElDED0SEAoMQUlfVV'
    'NFUl9JTkZPED4SEAoMU0VMRUNUX01PREVMED8=');

@$core.Deprecated('Use employeeRoleViewDescriptor instead')
const EmployeeRoleView$json = {
  '1': 'EmployeeRoleView',
  '2': [
    {'1': 'COUCH', '2': 0},
    {'1': 'MANAGER', '2': 1},
    {'1': 'ADMIN', '2': 2},
  ],
};

/// Descriptor for `EmployeeRoleView`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List employeeRoleViewDescriptor = $convert.base64Decode(
    'ChBFbXBsb3llZVJvbGVWaWV3EgkKBUNPVUNIEAASCwoHTUFOQUdFUhABEgkKBUFETUlOEAI=');

@$core.Deprecated('Use userViewDescriptor instead')
const UserView$json = {
  '1': 'UserView',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
    {'1': 'tg_id', '3': 2, '4': 1, '5': 3, '10': 'tgId'},
    {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.user.UserNameView', '10': 'name'},
    {'1': 'rights', '3': 4, '4': 1, '5': 11, '6': '.user.RightsView', '10': 'rights'},
    {'1': 'phone', '3': 5, '4': 1, '5': 9, '9': 0, '10': 'phone', '17': true},
    {'1': 'is_active', '3': 6, '4': 1, '5': 8, '10': 'isActive'},
    {'1': 'freeze', '3': 7, '4': 1, '5': 11, '6': '.user.FreezeView', '9': 1, '10': 'freeze', '17': true},
    {'1': 'subscriptions', '3': 8, '4': 3, '5': 11, '6': '.user.UserSubscriptionView', '10': 'subscriptions'},
    {'1': 'freeze_days', '3': 9, '4': 1, '5': 13, '10': 'freezeDays'},
    {'1': 'employee', '3': 12, '4': 1, '5': 11, '6': '.user.EmployeeView', '9': 2, '10': 'employee', '17': true},
    {'1': 'come_from', '3': 13, '4': 1, '5': 14, '6': '.user.SourceView', '9': 3, '10': 'comeFrom', '17': true},
    {'1': 'family', '3': 14, '4': 1, '5': 11, '6': '.user.FamilyView', '9': 4, '10': 'family', '17': true},
  ],
  '8': [
    {'1': '_phone'},
    {'1': '_freeze'},
    {'1': '_employee'},
    {'1': '_come_from'},
    {'1': '_family'},
  ],
};

/// Descriptor for `UserView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userViewDescriptor = $convert.base64Decode(
    'CghVc2VyVmlldxIcCgJpZBgBIAEoCzIMLmlkLk9iamVjdElkUgJpZBITCgV0Z19pZBgCIAEoA1'
    'IEdGdJZBImCgRuYW1lGAMgASgLMhIudXNlci5Vc2VyTmFtZVZpZXdSBG5hbWUSKAoGcmlnaHRz'
    'GAQgASgLMhAudXNlci5SaWdodHNWaWV3UgZyaWdodHMSGQoFcGhvbmUYBSABKAlIAFIFcGhvbm'
    'WIAQESGwoJaXNfYWN0aXZlGAYgASgIUghpc0FjdGl2ZRItCgZmcmVlemUYByABKAsyEC51c2Vy'
    'LkZyZWV6ZVZpZXdIAVIGZnJlZXpliAEBEkAKDXN1YnNjcmlwdGlvbnMYCCADKAsyGi51c2VyLl'
    'VzZXJTdWJzY3JpcHRpb25WaWV3Ug1zdWJzY3JpcHRpb25zEh8KC2ZyZWV6ZV9kYXlzGAkgASgN'
    'UgpmcmVlemVEYXlzEjMKCGVtcGxveWVlGAwgASgLMhIudXNlci5FbXBsb3llZVZpZXdIAlIIZW'
    '1wbG95ZWWIAQESMgoJY29tZV9mcm9tGA0gASgOMhAudXNlci5Tb3VyY2VWaWV3SANSCGNvbWVG'
    'cm9tiAEBEi0KBmZhbWlseRgOIAEoCzIQLnVzZXIuRmFtaWx5Vmlld0gEUgZmYW1pbHmIAQFCCA'
    'oGX3Bob25lQgkKB19mcmVlemVCCwoJX2VtcGxveWVlQgwKCl9jb21lX2Zyb21CCQoHX2ZhbWls'
    'eQ==');

@$core.Deprecated('Use userNameViewDescriptor instead')
const UserNameView$json = {
  '1': 'UserNameView',
  '2': [
    {'1': 'tg_user_name', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'tgUserName', '17': true},
    {'1': 'first_name', '3': 2, '4': 1, '5': 9, '10': 'firstName'},
    {'1': 'last_name', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'lastName', '17': true},
  ],
  '8': [
    {'1': '_tg_user_name'},
    {'1': '_last_name'},
  ],
};

/// Descriptor for `UserNameView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userNameViewDescriptor = $convert.base64Decode(
    'CgxVc2VyTmFtZVZpZXcSJQoMdGdfdXNlcl9uYW1lGAEgASgJSABSCnRnVXNlck5hbWWIAQESHQ'
    'oKZmlyc3RfbmFtZRgCIAEoCVIJZmlyc3ROYW1lEiAKCWxhc3RfbmFtZRgDIAEoCUgBUghsYXN0'
    'TmFtZYgBAUIPCg1fdGdfdXNlcl9uYW1lQgwKCl9sYXN0X25hbWU=');

@$core.Deprecated('Use freezeViewDescriptor instead')
const FreezeView$json = {
  '1': 'FreezeView',
  '2': [
    {'1': 'freeze_start', '3': 1, '4': 1, '5': 3, '10': 'freezeStart'},
    {'1': 'freeze_end', '3': 2, '4': 1, '5': 3, '10': 'freezeEnd'},
  ],
};

/// Descriptor for `FreezeView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List freezeViewDescriptor = $convert.base64Decode(
    'CgpGcmVlemVWaWV3EiEKDGZyZWV6ZV9zdGFydBgBIAEoA1ILZnJlZXplU3RhcnQSHQoKZnJlZX'
    'plX2VuZBgCIAEoA1IJZnJlZXplRW5k');

@$core.Deprecated('Use userSubscriptionViewDescriptor instead')
const UserSubscriptionView$json = {
  '1': 'UserSubscriptionView',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'id'},
    {'1': 'subscription_id', '3': 2, '4': 1, '5': 11, '6': '.id.ObjectId', '10': 'subscriptionId'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    {'1': 'items', '3': 4, '4': 1, '5': 13, '10': 'items'},
    {'1': 'days', '3': 5, '4': 1, '5': 13, '10': 'days'},
    {'1': 'status', '3': 6, '4': 1, '5': 11, '6': '.user.StatusView', '10': 'status'},
    {'1': 'price', '3': 7, '4': 1, '5': 3, '10': 'price'},
    {'1': 'tp', '3': 8, '4': 1, '5': 11, '6': '.subscription.SubscriptionTypeView', '10': 'tp'},
    {'1': 'balance', '3': 9, '4': 1, '5': 13, '10': 'balance'},
    {'1': 'locked_balance', '3': 10, '4': 1, '5': 13, '10': 'lockedBalance'},
    {'1': 'unlimited', '3': 11, '4': 1, '5': 8, '10': 'unlimited'},
    {'1': 'discount', '3': 12, '4': 1, '5': 3, '9': 0, '10': 'discount', '17': true},
    {'1': 'item_price', '3': 13, '4': 1, '5': 3, '9': 1, '10': 'itemPrice', '17': true},
  ],
  '8': [
    {'1': '_discount'},
    {'1': '_item_price'},
  ],
};

/// Descriptor for `UserSubscriptionView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userSubscriptionViewDescriptor = $convert.base64Decode(
    'ChRVc2VyU3Vic2NyaXB0aW9uVmlldxIcCgJpZBgBIAEoCzIMLmlkLk9iamVjdElkUgJpZBI1Cg'
    '9zdWJzY3JpcHRpb25faWQYAiABKAsyDC5pZC5PYmplY3RJZFIOc3Vic2NyaXB0aW9uSWQSEgoE'
    'bmFtZRgDIAEoCVIEbmFtZRIUCgVpdGVtcxgEIAEoDVIFaXRlbXMSEgoEZGF5cxgFIAEoDVIEZG'
    'F5cxIoCgZzdGF0dXMYBiABKAsyEC51c2VyLlN0YXR1c1ZpZXdSBnN0YXR1cxIUCgVwcmljZRgH'
    'IAEoA1IFcHJpY2USMgoCdHAYCCABKAsyIi5zdWJzY3JpcHRpb24uU3Vic2NyaXB0aW9uVHlwZV'
    'ZpZXdSAnRwEhgKB2JhbGFuY2UYCSABKA1SB2JhbGFuY2USJQoObG9ja2VkX2JhbGFuY2UYCiAB'
    'KA1SDWxvY2tlZEJhbGFuY2USHAoJdW5saW1pdGVkGAsgASgIUgl1bmxpbWl0ZWQSHwoIZGlzY2'
    '91bnQYDCABKANIAFIIZGlzY291bnSIAQESIgoKaXRlbV9wcmljZRgNIAEoA0gBUglpdGVtUHJp'
    'Y2WIAQFCCwoJX2Rpc2NvdW50Qg0KC19pdGVtX3ByaWNl');

@$core.Deprecated('Use employeeViewDescriptor instead')
const EmployeeView$json = {
  '1': 'EmployeeView',
  '2': [
    {'1': 'role', '3': 1, '4': 1, '5': 14, '6': '.user.EmployeeRoleView', '10': 'role'},
    {'1': 'description', '3': 2, '4': 1, '5': 9, '10': 'description'},
    {'1': 'reward', '3': 3, '4': 1, '5': 3, '10': 'reward'},
    {'1': 'rates', '3': 4, '4': 3, '5': 11, '6': '.user.RateView', '10': 'rates'},
  ],
};

/// Descriptor for `EmployeeView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List employeeViewDescriptor = $convert.base64Decode(
    'CgxFbXBsb3llZVZpZXcSKgoEcm9sZRgBIAEoDjIWLnVzZXIuRW1wbG95ZWVSb2xlVmlld1IEcm'
    '9sZRIgCgtkZXNjcmlwdGlvbhgCIAEoCVILZGVzY3JpcHRpb24SFgoGcmV3YXJkGAMgASgDUgZy'
    'ZXdhcmQSJAoFcmF0ZXMYBCADKAsyDi51c2VyLlJhdGVWaWV3UgVyYXRlcw==');

@$core.Deprecated('Use familyViewDescriptor instead')
const FamilyView$json = {
  '1': 'FamilyView',
  '2': [
    {'1': 'is_individual', '3': 2, '4': 1, '5': 8, '10': 'isIndividual'},
    {'1': 'payer', '3': 3, '4': 1, '5': 11, '6': '.user.UserView', '9': 0, '10': 'payer', '17': true},
    {'1': 'children', '3': 5, '4': 3, '5': 11, '6': '.user.UserView', '10': 'children'},
  ],
  '8': [
    {'1': '_payer'},
  ],
};

/// Descriptor for `FamilyView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List familyViewDescriptor = $convert.base64Decode(
    'CgpGYW1pbHlWaWV3EiMKDWlzX2luZGl2aWR1YWwYAiABKAhSDGlzSW5kaXZpZHVhbBIpCgVwYX'
    'llchgDIAEoCzIOLnVzZXIuVXNlclZpZXdIAFIFcGF5ZXKIAQESKgoIY2hpbGRyZW4YBSADKAsy'
    'Di51c2VyLlVzZXJWaWV3UghjaGlsZHJlbkIICgZfcGF5ZXI=');

@$core.Deprecated('Use rightsViewDescriptor instead')
const RightsView$json = {
  '1': 'RightsView',
  '2': [
    {'1': 'full', '3': 1, '4': 1, '5': 8, '10': 'full'},
    {'1': 'rights', '3': 2, '4': 3, '5': 14, '6': '.user.RuleView', '10': 'rights'},
  ],
};

/// Descriptor for `RightsView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rightsViewDescriptor = $convert.base64Decode(
    'CgpSaWdodHNWaWV3EhIKBGZ1bGwYASABKAhSBGZ1bGwSJgoGcmlnaHRzGAIgAygOMg4udXNlci'
    '5SdWxlVmlld1IGcmlnaHRz');

@$core.Deprecated('Use rateViewDescriptor instead')
const RateView$json = {
  '1': 'RateView',
  '2': [
    {'1': 'fix', '3': 1, '4': 1, '5': 11, '6': '.user.FixRateView', '9': 0, '10': 'fix'},
    {'1': 'group_training', '3': 2, '4': 1, '5': 11, '6': '.user.GroupTrainingRateView', '9': 0, '10': 'groupTraining'},
    {'1': 'personal_training', '3': 3, '4': 1, '5': 11, '6': '.user.PersonalTrainingRateView', '9': 0, '10': 'personalTraining'},
  ],
  '8': [
    {'1': 'rate_type'},
  ],
};

/// Descriptor for `RateView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rateViewDescriptor = $convert.base64Decode(
    'CghSYXRlVmlldxIlCgNmaXgYASABKAsyES51c2VyLkZpeFJhdGVWaWV3SABSA2ZpeBJECg5ncm'
    '91cF90cmFpbmluZxgCIAEoCzIbLnVzZXIuR3JvdXBUcmFpbmluZ1JhdGVWaWV3SABSDWdyb3Vw'
    'VHJhaW5pbmcSTQoRcGVyc29uYWxfdHJhaW5pbmcYAyABKAsyHi51c2VyLlBlcnNvbmFsVHJhaW'
    '5pbmdSYXRlVmlld0gAUhBwZXJzb25hbFRyYWluaW5nQgsKCXJhdGVfdHlwZQ==');

@$core.Deprecated('Use fixRateViewDescriptor instead')
const FixRateView$json = {
  '1': 'FixRateView',
  '2': [
    {'1': 'amount', '3': 1, '4': 1, '5': 3, '10': 'amount'},
    {'1': 'next_payment_date', '3': 2, '4': 1, '5': 3, '10': 'nextPaymentDate'},
    {'1': 'reward_interval', '3': 3, '4': 1, '5': 11, '6': '.user.IntervalView', '10': 'rewardInterval'},
  ],
};

/// Descriptor for `FixRateView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fixRateViewDescriptor = $convert.base64Decode(
    'CgtGaXhSYXRlVmlldxIWCgZhbW91bnQYASABKANSBmFtb3VudBIqChFuZXh0X3BheW1lbnRfZG'
    'F0ZRgCIAEoA1IPbmV4dFBheW1lbnREYXRlEjsKD3Jld2FyZF9pbnRlcnZhbBgDIAEoCzISLnVz'
    'ZXIuSW50ZXJ2YWxWaWV3Ug5yZXdhcmRJbnRlcnZhbA==');

@$core.Deprecated('Use groupTrainingRateViewDescriptor instead')
const GroupTrainingRateView$json = {
  '1': 'GroupTrainingRateView',
  '2': [
    {'1': 'percent', '3': 1, '4': 1, '5': 3, '10': 'percent'},
    {'1': 'min_reward', '3': 2, '4': 1, '5': 3, '10': 'minReward'},
  ],
};

/// Descriptor for `GroupTrainingRateView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupTrainingRateViewDescriptor = $convert.base64Decode(
    'ChVHcm91cFRyYWluaW5nUmF0ZVZpZXcSGAoHcGVyY2VudBgBIAEoA1IHcGVyY2VudBIdCgptaW'
    '5fcmV3YXJkGAIgASgDUgltaW5SZXdhcmQ=');

@$core.Deprecated('Use personalTrainingRateViewDescriptor instead')
const PersonalTrainingRateView$json = {
  '1': 'PersonalTrainingRateView',
  '2': [
    {'1': 'percent', '3': 1, '4': 1, '5': 3, '10': 'percent'},
  ],
};

/// Descriptor for `PersonalTrainingRateView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personalTrainingRateViewDescriptor = $convert.base64Decode(
    'ChhQZXJzb25hbFRyYWluaW5nUmF0ZVZpZXcSGAoHcGVyY2VudBgBIAEoA1IHcGVyY2VudA==');

@$core.Deprecated('Use intervalViewDescriptor instead')
const IntervalView$json = {
  '1': 'IntervalView',
  '2': [
    {'1': 'month_num', '3': 1, '4': 1, '5': 13, '10': 'monthNum'},
  ],
};

/// Descriptor for `IntervalView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List intervalViewDescriptor = $convert.base64Decode(
    'CgxJbnRlcnZhbFZpZXcSGwoJbW9udGhfbnVtGAEgASgNUghtb250aE51bQ==');

@$core.Deprecated('Use statusViewDescriptor instead')
const StatusView$json = {
  '1': 'StatusView',
  '2': [
    {'1': 'not_active', '3': 1, '4': 1, '5': 11, '6': '.user.NotActive', '9': 0, '10': 'notActive'},
    {'1': 'active', '3': 2, '4': 1, '5': 11, '6': '.user.ActiveStatusView', '9': 0, '10': 'active'},
  ],
  '8': [
    {'1': 'status_view'},
  ],
};

/// Descriptor for `StatusView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List statusViewDescriptor = $convert.base64Decode(
    'CgpTdGF0dXNWaWV3EjAKCm5vdF9hY3RpdmUYASABKAsyDy51c2VyLk5vdEFjdGl2ZUgAUglub3'
    'RBY3RpdmUSMAoGYWN0aXZlGAIgASgLMhYudXNlci5BY3RpdmVTdGF0dXNWaWV3SABSBmFjdGl2'
    'ZUINCgtzdGF0dXNfdmlldw==');

@$core.Deprecated('Use notActiveDescriptor instead')
const NotActive$json = {
  '1': 'NotActive',
};

/// Descriptor for `NotActive`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notActiveDescriptor = $convert.base64Decode(
    'CglOb3RBY3RpdmU=');

@$core.Deprecated('Use activeStatusViewDescriptor instead')
const ActiveStatusView$json = {
  '1': 'ActiveStatusView',
  '2': [
    {'1': 'start_date', '3': 1, '4': 1, '5': 3, '10': 'startDate'},
    {'1': 'end_date', '3': 2, '4': 1, '5': 3, '10': 'endDate'},
  ],
};

/// Descriptor for `ActiveStatusView`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List activeStatusViewDescriptor = $convert.base64Decode(
    'ChBBY3RpdmVTdGF0dXNWaWV3Eh0KCnN0YXJ0X2RhdGUYASABKANSCXN0YXJ0RGF0ZRIZCghlbm'
    'RfZGF0ZRgCIAEoA1IHZW5kRGF0ZQ==');

