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

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class SourceView extends $pb.ProtobufEnum {
  static const SourceView UNKNOWN = SourceView._(0, _omitEnumNames ? '' : 'UNKNOWN');
  static const SourceView WEBSITE = SourceView._(1, _omitEnumNames ? '' : 'WEBSITE');
  static const SourceView INSTAGRAM = SourceView._(2, _omitEnumNames ? '' : 'INSTAGRAM');
  static const SourceView VK = SourceView._(3, _omitEnumNames ? '' : 'VK');
  static const SourceView YANDEX_MAP = SourceView._(4, _omitEnumNames ? '' : 'YANDEX_MAP');
  static const SourceView YANDEX_DIRECT = SourceView._(5, _omitEnumNames ? '' : 'YANDEX_DIRECT');
  static const SourceView DIRECT_ADDS = SourceView._(6, _omitEnumNames ? '' : 'DIRECT_ADDS');
  static const SourceView VK_ADDS = SourceView._(7, _omitEnumNames ? '' : 'VK_ADDS');
  static const SourceView DOUBLE_GIS = SourceView._(8, _omitEnumNames ? '' : 'DOUBLE_GIS');
  static const SourceView AVITO = SourceView._(9, _omitEnumNames ? '' : 'AVITO');
  static const SourceView RECOMMENDATION = SourceView._(10, _omitEnumNames ? '' : 'RECOMMENDATION');
  static const SourceView OTHER = SourceView._(11, _omitEnumNames ? '' : 'OTHER');
  static const SourceView WEB_SEARCH = SourceView._(12, _omitEnumNames ? '' : 'WEB_SEARCH');
  static const SourceView OLD_BASE = SourceView._(13, _omitEnumNames ? '' : 'OLD_BASE');

  static const $core.List<SourceView> values = <SourceView> [
    UNKNOWN,
    WEBSITE,
    INSTAGRAM,
    VK,
    YANDEX_MAP,
    YANDEX_DIRECT,
    DIRECT_ADDS,
    VK_ADDS,
    DOUBLE_GIS,
    AVITO,
    RECOMMENDATION,
    OTHER,
    WEB_SEARCH,
    OLD_BASE,
  ];

  static final $core.List<SourceView?> _byValue = $pb.ProtobufEnum.$_initByValueList(values, 13);
  static SourceView? valueOf($core.int value) =>  value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SourceView._(super.value, super.name);
}

class RuleView extends $pb.ProtobufEnum {
  static const RuleView VIEW_PROFILE = RuleView._(0, _omitEnumNames ? '' : 'VIEW_PROFILE');
  static const RuleView VIEW_USERS = RuleView._(1, _omitEnumNames ? '' : 'VIEW_USERS');
  static const RuleView EDIT_USER_RIGHTS = RuleView._(2, _omitEnumNames ? '' : 'EDIT_USER_RIGHTS');
  static const RuleView BLOCK_USER = RuleView._(3, _omitEnumNames ? '' : 'BLOCK_USER');
  static const RuleView EDIT_USER_INFO = RuleView._(4, _omitEnumNames ? '' : 'EDIT_USER_INFO');
  static const RuleView EDIT_USER_SUBSCRIPTION = RuleView._(5, _omitEnumNames ? '' : 'EDIT_USER_SUBSCRIPTION');
  static const RuleView FREEZE_USERS = RuleView._(6, _omitEnumNames ? '' : 'FREEZE_USERS');
  static const RuleView CHANGE_BALANCE = RuleView._(7, _omitEnumNames ? '' : 'CHANGE_BALANCE');
  static const RuleView EDIT_MARKETING_INFO = RuleView._(8, _omitEnumNames ? '' : 'EDIT_MARKETING_INFO');
  static const RuleView EDIT_FAMILY = RuleView._(9, _omitEnumNames ? '' : 'EDIT_FAMILY');
  static const RuleView VIEW_FAMILY = RuleView._(10, _omitEnumNames ? '' : 'VIEW_FAMILY');
  static const RuleView EDIT_AI_PROMPT = RuleView._(11, _omitEnumNames ? '' : 'EDIT_AI_PROMPT');
  static const RuleView VIEW_USER_COMMENTS = RuleView._(12, _omitEnumNames ? '' : 'VIEW_USER_COMMENTS');
  static const RuleView EDIT_USER_COMMENTS = RuleView._(13, _omitEnumNames ? '' : 'EDIT_USER_COMMENTS');
  static const RuleView DELETE_USER_COMMENTS = RuleView._(14, _omitEnumNames ? '' : 'DELETE_USER_COMMENTS');
  static const RuleView EDIT_TRAINING = RuleView._(15, _omitEnumNames ? '' : 'EDIT_TRAINING');
  static const RuleView CREATE_TRAINING = RuleView._(16, _omitEnumNames ? '' : 'CREATE_TRAINING');
  static const RuleView EDIT_TRAINING_CLIENTS_LIST = RuleView._(17, _omitEnumNames ? '' : 'EDIT_TRAINING_CLIENTS_LIST');
  static const RuleView SET_KEEP_OPEN = RuleView._(18, _omitEnumNames ? '' : 'SET_KEEP_OPEN');
  static const RuleView SET_FREE = RuleView._(19, _omitEnumNames ? '' : 'SET_FREE');
  static const RuleView EDIT_SCHEDULE = RuleView._(20, _omitEnumNames ? '' : 'EDIT_SCHEDULE');
  static const RuleView CANCEL_TRAINING = RuleView._(21, _omitEnumNames ? '' : 'CANCEL_TRAINING');
  static const RuleView REMOVE_TRAINING = RuleView._(22, _omitEnumNames ? '' : 'REMOVE_TRAINING');
  static const RuleView EDIT_TRAINING_COUCH = RuleView._(23, _omitEnumNames ? '' : 'EDIT_TRAINING_COUCH');
  static const RuleView SCHEDULE_GROUP_TRAINING = RuleView._(24, _omitEnumNames ? '' : 'SCHEDULE_GROUP_TRAINING');
  static const RuleView SCHEDULE_PERSONAL_TRAINING = RuleView._(25, _omitEnumNames ? '' : 'SCHEDULE_PERSONAL_TRAINING');
  static const RuleView SCHEDULE_SUB_RENT = RuleView._(26, _omitEnumNames ? '' : 'SCHEDULE_SUB_RENT');
  static const RuleView SELECT_PERSONAL_INSTRUCTOR = RuleView._(27, _omitEnumNames ? '' : 'SELECT_PERSONAL_INSTRUCTOR');
  static const RuleView VIEW_ALL_TRAININGS = RuleView._(28, _omitEnumNames ? '' : 'VIEW_ALL_TRAININGS');
  static const RuleView CHANGE_TRAINING_SLOT = RuleView._(29, _omitEnumNames ? '' : 'CHANGE_TRAINING_SLOT');
  static const RuleView CREATE_SUBSCRIPTION = RuleView._(30, _omitEnumNames ? '' : 'CREATE_SUBSCRIPTION');
  static const RuleView EDIT_SUBSCRIPTION = RuleView._(31, _omitEnumNames ? '' : 'EDIT_SUBSCRIPTION');
  static const RuleView SELL_SUBSCRIPTION = RuleView._(32, _omitEnumNames ? '' : 'SELL_SUBSCRIPTION');
  static const RuleView FREE_SELL = RuleView._(33, _omitEnumNames ? '' : 'FREE_SELL');
  static const RuleView SUB_RENT = RuleView._(34, _omitEnumNames ? '' : 'SUB_RENT');
  static const RuleView VIEW_FINANCE = RuleView._(35, _omitEnumNames ? '' : 'VIEW_FINANCE');
  static const RuleView MAKE_PAYMENT = RuleView._(36, _omitEnumNames ? '' : 'MAKE_PAYMENT');
  static const RuleView MAKE_DEPOSIT = RuleView._(37, _omitEnumNames ? '' : 'MAKE_DEPOSIT');
  static const RuleView FINANCE_HISTORICAL_DATE = RuleView._(38, _omitEnumNames ? '' : 'FINANCE_HISTORICAL_DATE');
  static const RuleView DELETE_HISTORY = RuleView._(39, _omitEnumNames ? '' : 'DELETE_HISTORY');
  static const RuleView VIEW_EMPLOYEES = RuleView._(40, _omitEnumNames ? '' : 'VIEW_EMPLOYEES');
  static const RuleView EDIT_EMPLOYEE = RuleView._(41, _omitEnumNames ? '' : 'EDIT_EMPLOYEE');
  static const RuleView EDIT_EMPLOYEE_RATES = RuleView._(42, _omitEnumNames ? '' : 'EDIT_EMPLOYEE_RATES');
  static const RuleView VIEW_LOGS = RuleView._(43, _omitEnumNames ? '' : 'VIEW_LOGS');
  static const RuleView CREATE_COUCH = RuleView._(44, _omitEnumNames ? '' : 'CREATE_COUCH');
  static const RuleView EDIT_COUCH = RuleView._(45, _omitEnumNames ? '' : 'EDIT_COUCH');
  static const RuleView VIEW_COUCH_RATES = RuleView._(46, _omitEnumNames ? '' : 'VIEW_COUCH_RATES');
  static const RuleView VIEW_STATISTICS = RuleView._(47, _omitEnumNames ? '' : 'VIEW_STATISTICS');
  static const RuleView SYSTEM = RuleView._(48, _omitEnumNames ? '' : 'SYSTEM');
  static const RuleView VIEW_REWARDS = RuleView._(49, _omitEnumNames ? '' : 'VIEW_REWARDS');
  static const RuleView RECALCULATE_REWARDS = RuleView._(50, _omitEnumNames ? '' : 'RECALCULATE_REWARDS');
  static const RuleView VIEW_MARKETING_INFO = RuleView._(51, _omitEnumNames ? '' : 'VIEW_MARKETING_INFO');
  static const RuleView CREATE_REQUEST = RuleView._(52, _omitEnumNames ? '' : 'CREATE_REQUEST');
  static const RuleView REQUESTS_HISTORY = RuleView._(53, _omitEnumNames ? '' : 'REQUESTS_HISTORY');
  static const RuleView RECEIVE_NOTIFICATIONS_ABOUT_SUBSCRIPTIONS = RuleView._(54, _omitEnumNames ? '' : 'RECEIVE_NOTIFICATIONS_ABOUT_SUBSCRIPTIONS');
  static const RuleView RECEIVE_NOTIFICATIONS_ABOUT_BIRTHDAYS = RuleView._(55, _omitEnumNames ? '' : 'RECEIVE_NOTIFICATIONS_ABOUT_BIRTHDAYS');
  static const RuleView RECEIVE_AI_NOTIFICATIONS = RuleView._(56, _omitEnumNames ? '' : 'RECEIVE_AI_NOTIFICATIONS');
  static const RuleView MINI_APP = RuleView._(57, _omitEnumNames ? '' : 'MINI_APP');
  static const RuleView BUY_SUBSCRIPTION = RuleView._(58, _omitEnumNames ? '' : 'BUY_SUBSCRIPTION');
  static const RuleView VIEW_HIDDEN_PROGRAMS = RuleView._(59, _omitEnumNames ? '' : 'VIEW_HIDDEN_PROGRAMS');
  static const RuleView HISTORY_VIEWER = RuleView._(60, _omitEnumNames ? '' : 'HISTORY_VIEWER');
  static const RuleView AI_STATISTIC = RuleView._(61, _omitEnumNames ? '' : 'AI_STATISTIC');
  static const RuleView AI_USER_INFO = RuleView._(62, _omitEnumNames ? '' : 'AI_USER_INFO');
  static const RuleView SELECT_MODEL = RuleView._(63, _omitEnumNames ? '' : 'SELECT_MODEL');

  static const $core.List<RuleView> values = <RuleView> [
    VIEW_PROFILE,
    VIEW_USERS,
    EDIT_USER_RIGHTS,
    BLOCK_USER,
    EDIT_USER_INFO,
    EDIT_USER_SUBSCRIPTION,
    FREEZE_USERS,
    CHANGE_BALANCE,
    EDIT_MARKETING_INFO,
    EDIT_FAMILY,
    VIEW_FAMILY,
    EDIT_AI_PROMPT,
    VIEW_USER_COMMENTS,
    EDIT_USER_COMMENTS,
    DELETE_USER_COMMENTS,
    EDIT_TRAINING,
    CREATE_TRAINING,
    EDIT_TRAINING_CLIENTS_LIST,
    SET_KEEP_OPEN,
    SET_FREE,
    EDIT_SCHEDULE,
    CANCEL_TRAINING,
    REMOVE_TRAINING,
    EDIT_TRAINING_COUCH,
    SCHEDULE_GROUP_TRAINING,
    SCHEDULE_PERSONAL_TRAINING,
    SCHEDULE_SUB_RENT,
    SELECT_PERSONAL_INSTRUCTOR,
    VIEW_ALL_TRAININGS,
    CHANGE_TRAINING_SLOT,
    CREATE_SUBSCRIPTION,
    EDIT_SUBSCRIPTION,
    SELL_SUBSCRIPTION,
    FREE_SELL,
    SUB_RENT,
    VIEW_FINANCE,
    MAKE_PAYMENT,
    MAKE_DEPOSIT,
    FINANCE_HISTORICAL_DATE,
    DELETE_HISTORY,
    VIEW_EMPLOYEES,
    EDIT_EMPLOYEE,
    EDIT_EMPLOYEE_RATES,
    VIEW_LOGS,
    CREATE_COUCH,
    EDIT_COUCH,
    VIEW_COUCH_RATES,
    VIEW_STATISTICS,
    SYSTEM,
    VIEW_REWARDS,
    RECALCULATE_REWARDS,
    VIEW_MARKETING_INFO,
    CREATE_REQUEST,
    REQUESTS_HISTORY,
    RECEIVE_NOTIFICATIONS_ABOUT_SUBSCRIPTIONS,
    RECEIVE_NOTIFICATIONS_ABOUT_BIRTHDAYS,
    RECEIVE_AI_NOTIFICATIONS,
    MINI_APP,
    BUY_SUBSCRIPTION,
    VIEW_HIDDEN_PROGRAMS,
    HISTORY_VIEWER,
    AI_STATISTIC,
    AI_USER_INFO,
    SELECT_MODEL,
  ];

  static final $core.List<RuleView?> _byValue = $pb.ProtobufEnum.$_initByValueList(values, 63);
  static RuleView? valueOf($core.int value) =>  value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RuleView._(super.value, super.name);
}

class EmployeeRoleView extends $pb.ProtobufEnum {
  static const EmployeeRoleView COUCH = EmployeeRoleView._(0, _omitEnumNames ? '' : 'COUCH');
  static const EmployeeRoleView MANAGER = EmployeeRoleView._(1, _omitEnumNames ? '' : 'MANAGER');
  static const EmployeeRoleView ADMIN = EmployeeRoleView._(2, _omitEnumNames ? '' : 'ADMIN');

  static const $core.List<EmployeeRoleView> values = <EmployeeRoleView> [
    COUCH,
    MANAGER,
    ADMIN,
  ];

  static final $core.List<EmployeeRoleView?> _byValue = $pb.ProtobufEnum.$_initByValueList(values, 2);
  static EmployeeRoleView? valueOf($core.int value) =>  value < 0 || value >= _byValue.length ? null : _byValue[value];

  const EmployeeRoleView._(super.value, super.name);
}


const $core.bool _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
