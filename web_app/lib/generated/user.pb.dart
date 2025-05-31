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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'id.pb.dart' as $3;
import 'subscription.pb.dart' as $4;
import 'user.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'user.pbenum.dart';

class UserView extends $pb.GeneratedMessage {
  factory UserView({
    $3.ObjectId? id,
    $fixnum.Int64? tgId,
    UserNameView? name,
    RightsView? rights,
    $core.String? phone,
    $core.bool? isActive,
    FreezeView? freeze_7,
    $core.Iterable<UserSubscriptionView>? subscriptions,
    $core.int? freezeDays,
    EmployeeView? employee,
    SourceView? comeFrom,
    FamilyView? family,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (tgId != null) result.tgId = tgId;
    if (name != null) result.name = name;
    if (rights != null) result.rights = rights;
    if (phone != null) result.phone = phone;
    if (isActive != null) result.isActive = isActive;
    if (freeze_7 != null) result.freeze_7 = freeze_7;
    if (subscriptions != null) result.subscriptions.addAll(subscriptions);
    if (freezeDays != null) result.freezeDays = freezeDays;
    if (employee != null) result.employee = employee;
    if (comeFrom != null) result.comeFrom = comeFrom;
    if (family != null) result.family = family;
    return result;
  }

  UserView._();

  factory UserView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UserView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aOM<$3.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $3.ObjectId.create)
    ..aInt64(2, _omitFieldNames ? '' : 'tgId')
    ..aOM<UserNameView>(3, _omitFieldNames ? '' : 'name', subBuilder: UserNameView.create)
    ..aOM<RightsView>(4, _omitFieldNames ? '' : 'rights', subBuilder: RightsView.create)
    ..aOS(5, _omitFieldNames ? '' : 'phone')
    ..aOB(6, _omitFieldNames ? '' : 'isActive')
    ..aOM<FreezeView>(7, _omitFieldNames ? '' : 'freeze', subBuilder: FreezeView.create)
    ..pc<UserSubscriptionView>(8, _omitFieldNames ? '' : 'subscriptions', $pb.PbFieldType.PM, subBuilder: UserSubscriptionView.create)
    ..a<$core.int>(9, _omitFieldNames ? '' : 'freezeDays', $pb.PbFieldType.OU3)
    ..aOM<EmployeeView>(12, _omitFieldNames ? '' : 'employee', subBuilder: EmployeeView.create)
    ..e<SourceView>(13, _omitFieldNames ? '' : 'comeFrom', $pb.PbFieldType.OE, defaultOrMaker: SourceView.UNKNOWN, valueOf: SourceView.valueOf, enumValues: SourceView.values)
    ..aOM<FamilyView>(14, _omitFieldNames ? '' : 'family', subBuilder: FamilyView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserView clone() => UserView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserView copyWith(void Function(UserView) updates) => super.copyWith((message) => updates(message as UserView)) as UserView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserView create() => UserView._();
  @$core.override
  UserView createEmptyInstance() => create();
  static $pb.PbList<UserView> createRepeated() => $pb.PbList<UserView>();
  @$core.pragma('dart2js:noInline')
  static UserView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserView>(create);
  static UserView? _defaultInstance;

  @$pb.TagNumber(1)
  $3.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($3.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $3.ObjectId ensureId() => $_ensure(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get tgId => $_getI64(1);
  @$pb.TagNumber(2)
  set tgId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTgId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTgId() => $_clearField(2);

  @$pb.TagNumber(3)
  UserNameView get name => $_getN(2);
  @$pb.TagNumber(3)
  set name(UserNameView value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);
  @$pb.TagNumber(3)
  UserNameView ensureName() => $_ensure(2);

  @$pb.TagNumber(4)
  RightsView get rights => $_getN(3);
  @$pb.TagNumber(4)
  set rights(RightsView value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasRights() => $_has(3);
  @$pb.TagNumber(4)
  void clearRights() => $_clearField(4);
  @$pb.TagNumber(4)
  RightsView ensureRights() => $_ensure(3);

  @$pb.TagNumber(5)
  $core.String get phone => $_getSZ(4);
  @$pb.TagNumber(5)
  set phone($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPhone() => $_has(4);
  @$pb.TagNumber(5)
  void clearPhone() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get isActive => $_getBF(5);
  @$pb.TagNumber(6)
  set isActive($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasIsActive() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsActive() => $_clearField(6);

  @$pb.TagNumber(7)
  FreezeView get freeze_7 => $_getN(6);
  @$pb.TagNumber(7)
  set freeze_7(FreezeView value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasFreeze_7() => $_has(6);
  @$pb.TagNumber(7)
  void clearFreeze_7() => $_clearField(7);
  @$pb.TagNumber(7)
  FreezeView ensureFreeze_7() => $_ensure(6);

  @$pb.TagNumber(8)
  $pb.PbList<UserSubscriptionView> get subscriptions => $_getList(7);

  @$pb.TagNumber(9)
  $core.int get freezeDays => $_getIZ(8);
  @$pb.TagNumber(9)
  set freezeDays($core.int value) => $_setUnsignedInt32(8, value);
  @$pb.TagNumber(9)
  $core.bool hasFreezeDays() => $_has(8);
  @$pb.TagNumber(9)
  void clearFreezeDays() => $_clearField(9);

  @$pb.TagNumber(12)
  EmployeeView get employee => $_getN(9);
  @$pb.TagNumber(12)
  set employee(EmployeeView value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasEmployee() => $_has(9);
  @$pb.TagNumber(12)
  void clearEmployee() => $_clearField(12);
  @$pb.TagNumber(12)
  EmployeeView ensureEmployee() => $_ensure(9);

  @$pb.TagNumber(13)
  SourceView get comeFrom => $_getN(10);
  @$pb.TagNumber(13)
  set comeFrom(SourceView value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasComeFrom() => $_has(10);
  @$pb.TagNumber(13)
  void clearComeFrom() => $_clearField(13);

  @$pb.TagNumber(14)
  FamilyView get family => $_getN(11);
  @$pb.TagNumber(14)
  set family(FamilyView value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasFamily() => $_has(11);
  @$pb.TagNumber(14)
  void clearFamily() => $_clearField(14);
  @$pb.TagNumber(14)
  FamilyView ensureFamily() => $_ensure(11);
}

class UserNameView extends $pb.GeneratedMessage {
  factory UserNameView({
    $core.String? tgUserName,
    $core.String? firstName,
    $core.String? lastName,
  }) {
    final result = create();
    if (tgUserName != null) result.tgUserName = tgUserName;
    if (firstName != null) result.firstName = firstName;
    if (lastName != null) result.lastName = lastName;
    return result;
  }

  UserNameView._();

  factory UserNameView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UserNameView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserNameView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'tgUserName')
    ..aOS(2, _omitFieldNames ? '' : 'firstName')
    ..aOS(3, _omitFieldNames ? '' : 'lastName')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserNameView clone() => UserNameView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserNameView copyWith(void Function(UserNameView) updates) => super.copyWith((message) => updates(message as UserNameView)) as UserNameView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserNameView create() => UserNameView._();
  @$core.override
  UserNameView createEmptyInstance() => create();
  static $pb.PbList<UserNameView> createRepeated() => $pb.PbList<UserNameView>();
  @$core.pragma('dart2js:noInline')
  static UserNameView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserNameView>(create);
  static UserNameView? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get tgUserName => $_getSZ(0);
  @$pb.TagNumber(1)
  set tgUserName($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTgUserName() => $_has(0);
  @$pb.TagNumber(1)
  void clearTgUserName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get firstName => $_getSZ(1);
  @$pb.TagNumber(2)
  set firstName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFirstName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFirstName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get lastName => $_getSZ(2);
  @$pb.TagNumber(3)
  set lastName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLastName() => $_has(2);
  @$pb.TagNumber(3)
  void clearLastName() => $_clearField(3);
}

class FreezeView extends $pb.GeneratedMessage {
  factory FreezeView({
    $fixnum.Int64? freezeStart,
    $fixnum.Int64? freezeEnd,
  }) {
    final result = create();
    if (freezeStart != null) result.freezeStart = freezeStart;
    if (freezeEnd != null) result.freezeEnd = freezeEnd;
    return result;
  }

  FreezeView._();

  factory FreezeView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory FreezeView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FreezeView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aInt64(1, _omitFieldNames ? '' : 'freezeStart')
    ..aInt64(2, _omitFieldNames ? '' : 'freezeEnd')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FreezeView clone() => FreezeView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FreezeView copyWith(void Function(FreezeView) updates) => super.copyWith((message) => updates(message as FreezeView)) as FreezeView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FreezeView create() => FreezeView._();
  @$core.override
  FreezeView createEmptyInstance() => create();
  static $pb.PbList<FreezeView> createRepeated() => $pb.PbList<FreezeView>();
  @$core.pragma('dart2js:noInline')
  static FreezeView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FreezeView>(create);
  static FreezeView? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get freezeStart => $_getI64(0);
  @$pb.TagNumber(1)
  set freezeStart($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFreezeStart() => $_has(0);
  @$pb.TagNumber(1)
  void clearFreezeStart() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get freezeEnd => $_getI64(1);
  @$pb.TagNumber(2)
  set freezeEnd($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFreezeEnd() => $_has(1);
  @$pb.TagNumber(2)
  void clearFreezeEnd() => $_clearField(2);
}

class UserSubscriptionView extends $pb.GeneratedMessage {
  factory UserSubscriptionView({
    $3.ObjectId? id,
    $3.ObjectId? subscriptionId,
    $core.String? name,
    $core.int? items,
    $core.int? days,
    StatusView? status,
    $fixnum.Int64? price,
    $4.SubscriptionTypeView? tp,
    $core.int? balance,
    $core.int? lockedBalance,
    $core.bool? unlimited,
    $fixnum.Int64? discount,
    $fixnum.Int64? itemPrice,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (subscriptionId != null) result.subscriptionId = subscriptionId;
    if (name != null) result.name = name;
    if (items != null) result.items = items;
    if (days != null) result.days = days;
    if (status != null) result.status = status;
    if (price != null) result.price = price;
    if (tp != null) result.tp = tp;
    if (balance != null) result.balance = balance;
    if (lockedBalance != null) result.lockedBalance = lockedBalance;
    if (unlimited != null) result.unlimited = unlimited;
    if (discount != null) result.discount = discount;
    if (itemPrice != null) result.itemPrice = itemPrice;
    return result;
  }

  UserSubscriptionView._();

  factory UserSubscriptionView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UserSubscriptionView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserSubscriptionView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aOM<$3.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $3.ObjectId.create)
    ..aOM<$3.ObjectId>(2, _omitFieldNames ? '' : 'subscriptionId', subBuilder: $3.ObjectId.create)
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..a<$core.int>(4, _omitFieldNames ? '' : 'items', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, _omitFieldNames ? '' : 'days', $pb.PbFieldType.OU3)
    ..aOM<StatusView>(6, _omitFieldNames ? '' : 'status', subBuilder: StatusView.create)
    ..aInt64(7, _omitFieldNames ? '' : 'price')
    ..aOM<$4.SubscriptionTypeView>(8, _omitFieldNames ? '' : 'tp', subBuilder: $4.SubscriptionTypeView.create)
    ..a<$core.int>(9, _omitFieldNames ? '' : 'balance', $pb.PbFieldType.OU3)
    ..a<$core.int>(10, _omitFieldNames ? '' : 'lockedBalance', $pb.PbFieldType.OU3)
    ..aOB(11, _omitFieldNames ? '' : 'unlimited')
    ..aInt64(12, _omitFieldNames ? '' : 'discount')
    ..aInt64(13, _omitFieldNames ? '' : 'itemPrice')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserSubscriptionView clone() => UserSubscriptionView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserSubscriptionView copyWith(void Function(UserSubscriptionView) updates) => super.copyWith((message) => updates(message as UserSubscriptionView)) as UserSubscriptionView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserSubscriptionView create() => UserSubscriptionView._();
  @$core.override
  UserSubscriptionView createEmptyInstance() => create();
  static $pb.PbList<UserSubscriptionView> createRepeated() => $pb.PbList<UserSubscriptionView>();
  @$core.pragma('dart2js:noInline')
  static UserSubscriptionView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserSubscriptionView>(create);
  static UserSubscriptionView? _defaultInstance;

  @$pb.TagNumber(1)
  $3.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($3.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $3.ObjectId ensureId() => $_ensure(0);

  @$pb.TagNumber(2)
  $3.ObjectId get subscriptionId => $_getN(1);
  @$pb.TagNumber(2)
  set subscriptionId($3.ObjectId value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasSubscriptionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSubscriptionId() => $_clearField(2);
  @$pb.TagNumber(2)
  $3.ObjectId ensureSubscriptionId() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get items => $_getIZ(3);
  @$pb.TagNumber(4)
  set items($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasItems() => $_has(3);
  @$pb.TagNumber(4)
  void clearItems() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get days => $_getIZ(4);
  @$pb.TagNumber(5)
  set days($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDays() => $_has(4);
  @$pb.TagNumber(5)
  void clearDays() => $_clearField(5);

  @$pb.TagNumber(6)
  StatusView get status => $_getN(5);
  @$pb.TagNumber(6)
  set status(StatusView value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasStatus() => $_has(5);
  @$pb.TagNumber(6)
  void clearStatus() => $_clearField(6);
  @$pb.TagNumber(6)
  StatusView ensureStatus() => $_ensure(5);

  @$pb.TagNumber(7)
  $fixnum.Int64 get price => $_getI64(6);
  @$pb.TagNumber(7)
  set price($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasPrice() => $_has(6);
  @$pb.TagNumber(7)
  void clearPrice() => $_clearField(7);

  @$pb.TagNumber(8)
  $4.SubscriptionTypeView get tp => $_getN(7);
  @$pb.TagNumber(8)
  set tp($4.SubscriptionTypeView value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasTp() => $_has(7);
  @$pb.TagNumber(8)
  void clearTp() => $_clearField(8);
  @$pb.TagNumber(8)
  $4.SubscriptionTypeView ensureTp() => $_ensure(7);

  @$pb.TagNumber(9)
  $core.int get balance => $_getIZ(8);
  @$pb.TagNumber(9)
  set balance($core.int value) => $_setUnsignedInt32(8, value);
  @$pb.TagNumber(9)
  $core.bool hasBalance() => $_has(8);
  @$pb.TagNumber(9)
  void clearBalance() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.int get lockedBalance => $_getIZ(9);
  @$pb.TagNumber(10)
  set lockedBalance($core.int value) => $_setUnsignedInt32(9, value);
  @$pb.TagNumber(10)
  $core.bool hasLockedBalance() => $_has(9);
  @$pb.TagNumber(10)
  void clearLockedBalance() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get unlimited => $_getBF(10);
  @$pb.TagNumber(11)
  set unlimited($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasUnlimited() => $_has(10);
  @$pb.TagNumber(11)
  void clearUnlimited() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get discount => $_getI64(11);
  @$pb.TagNumber(12)
  set discount($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasDiscount() => $_has(11);
  @$pb.TagNumber(12)
  void clearDiscount() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get itemPrice => $_getI64(12);
  @$pb.TagNumber(13)
  set itemPrice($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasItemPrice() => $_has(12);
  @$pb.TagNumber(13)
  void clearItemPrice() => $_clearField(13);
}

class EmployeeView extends $pb.GeneratedMessage {
  factory EmployeeView({
    EmployeeRoleView? role,
    $core.String? description,
    $fixnum.Int64? reward,
    $core.Iterable<RateView>? rates,
  }) {
    final result = create();
    if (role != null) result.role = role;
    if (description != null) result.description = description;
    if (reward != null) result.reward = reward;
    if (rates != null) result.rates.addAll(rates);
    return result;
  }

  EmployeeView._();

  factory EmployeeView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory EmployeeView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EmployeeView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..e<EmployeeRoleView>(1, _omitFieldNames ? '' : 'role', $pb.PbFieldType.OE, defaultOrMaker: EmployeeRoleView.COUCH, valueOf: EmployeeRoleView.valueOf, enumValues: EmployeeRoleView.values)
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aInt64(3, _omitFieldNames ? '' : 'reward')
    ..pc<RateView>(4, _omitFieldNames ? '' : 'rates', $pb.PbFieldType.PM, subBuilder: RateView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EmployeeView clone() => EmployeeView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EmployeeView copyWith(void Function(EmployeeView) updates) => super.copyWith((message) => updates(message as EmployeeView)) as EmployeeView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EmployeeView create() => EmployeeView._();
  @$core.override
  EmployeeView createEmptyInstance() => create();
  static $pb.PbList<EmployeeView> createRepeated() => $pb.PbList<EmployeeView>();
  @$core.pragma('dart2js:noInline')
  static EmployeeView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EmployeeView>(create);
  static EmployeeView? _defaultInstance;

  @$pb.TagNumber(1)
  EmployeeRoleView get role => $_getN(0);
  @$pb.TagNumber(1)
  set role(EmployeeRoleView value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearRole() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get description => $_getSZ(1);
  @$pb.TagNumber(2)
  set description($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDescription() => $_has(1);
  @$pb.TagNumber(2)
  void clearDescription() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get reward => $_getI64(2);
  @$pb.TagNumber(3)
  set reward($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasReward() => $_has(2);
  @$pb.TagNumber(3)
  void clearReward() => $_clearField(3);

  @$pb.TagNumber(4)
  $pb.PbList<RateView> get rates => $_getList(3);
}

class FamilyView extends $pb.GeneratedMessage {
  factory FamilyView({
    $core.bool? isIndividual,
    UserView? payer,
    $core.Iterable<UserView>? children,
  }) {
    final result = create();
    if (isIndividual != null) result.isIndividual = isIndividual;
    if (payer != null) result.payer = payer;
    if (children != null) result.children.addAll(children);
    return result;
  }

  FamilyView._();

  factory FamilyView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory FamilyView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FamilyView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aOB(2, _omitFieldNames ? '' : 'isIndividual')
    ..aOM<UserView>(3, _omitFieldNames ? '' : 'payer', subBuilder: UserView.create)
    ..pc<UserView>(5, _omitFieldNames ? '' : 'children', $pb.PbFieldType.PM, subBuilder: UserView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FamilyView clone() => FamilyView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FamilyView copyWith(void Function(FamilyView) updates) => super.copyWith((message) => updates(message as FamilyView)) as FamilyView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FamilyView create() => FamilyView._();
  @$core.override
  FamilyView createEmptyInstance() => create();
  static $pb.PbList<FamilyView> createRepeated() => $pb.PbList<FamilyView>();
  @$core.pragma('dart2js:noInline')
  static FamilyView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FamilyView>(create);
  static FamilyView? _defaultInstance;

  @$pb.TagNumber(2)
  $core.bool get isIndividual => $_getBF(0);
  @$pb.TagNumber(2)
  set isIndividual($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(2)
  $core.bool hasIsIndividual() => $_has(0);
  @$pb.TagNumber(2)
  void clearIsIndividual() => $_clearField(2);

  @$pb.TagNumber(3)
  UserView get payer => $_getN(1);
  @$pb.TagNumber(3)
  set payer(UserView value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasPayer() => $_has(1);
  @$pb.TagNumber(3)
  void clearPayer() => $_clearField(3);
  @$pb.TagNumber(3)
  UserView ensurePayer() => $_ensure(1);

  @$pb.TagNumber(5)
  $pb.PbList<UserView> get children => $_getList(2);
}

class RightsView extends $pb.GeneratedMessage {
  factory RightsView({
    $core.bool? full,
    $core.Iterable<RuleView>? rights,
  }) {
    final result = create();
    if (full != null) result.full = full;
    if (rights != null) result.rights.addAll(rights);
    return result;
  }

  RightsView._();

  factory RightsView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory RightsView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RightsView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'full')
    ..pc<RuleView>(2, _omitFieldNames ? '' : 'rights', $pb.PbFieldType.KE, valueOf: RuleView.valueOf, enumValues: RuleView.values, defaultEnumValue: RuleView.VIEW_PROFILE)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RightsView clone() => RightsView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RightsView copyWith(void Function(RightsView) updates) => super.copyWith((message) => updates(message as RightsView)) as RightsView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RightsView create() => RightsView._();
  @$core.override
  RightsView createEmptyInstance() => create();
  static $pb.PbList<RightsView> createRepeated() => $pb.PbList<RightsView>();
  @$core.pragma('dart2js:noInline')
  static RightsView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RightsView>(create);
  static RightsView? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get full => $_getBF(0);
  @$pb.TagNumber(1)
  set full($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFull() => $_has(0);
  @$pb.TagNumber(1)
  void clearFull() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<RuleView> get rights => $_getList(1);
}

enum RateView_RateType {
  fix, 
  groupTraining, 
  personalTraining, 
  notSet
}

class RateView extends $pb.GeneratedMessage {
  factory RateView({
    FixRateView? fix,
    GroupTrainingRateView? groupTraining,
    PersonalTrainingRateView? personalTraining,
  }) {
    final result = create();
    if (fix != null) result.fix = fix;
    if (groupTraining != null) result.groupTraining = groupTraining;
    if (personalTraining != null) result.personalTraining = personalTraining;
    return result;
  }

  RateView._();

  factory RateView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory RateView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, RateView_RateType> _RateView_RateTypeByTag = {
    1 : RateView_RateType.fix,
    2 : RateView_RateType.groupTraining,
    3 : RateView_RateType.personalTraining,
    0 : RateView_RateType.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RateView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<FixRateView>(1, _omitFieldNames ? '' : 'fix', subBuilder: FixRateView.create)
    ..aOM<GroupTrainingRateView>(2, _omitFieldNames ? '' : 'groupTraining', subBuilder: GroupTrainingRateView.create)
    ..aOM<PersonalTrainingRateView>(3, _omitFieldNames ? '' : 'personalTraining', subBuilder: PersonalTrainingRateView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RateView clone() => RateView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RateView copyWith(void Function(RateView) updates) => super.copyWith((message) => updates(message as RateView)) as RateView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RateView create() => RateView._();
  @$core.override
  RateView createEmptyInstance() => create();
  static $pb.PbList<RateView> createRepeated() => $pb.PbList<RateView>();
  @$core.pragma('dart2js:noInline')
  static RateView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RateView>(create);
  static RateView? _defaultInstance;

  RateView_RateType whichRateType() => _RateView_RateTypeByTag[$_whichOneof(0)]!;
  void clearRateType() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  FixRateView get fix => $_getN(0);
  @$pb.TagNumber(1)
  set fix(FixRateView value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasFix() => $_has(0);
  @$pb.TagNumber(1)
  void clearFix() => $_clearField(1);
  @$pb.TagNumber(1)
  FixRateView ensureFix() => $_ensure(0);

  @$pb.TagNumber(2)
  GroupTrainingRateView get groupTraining => $_getN(1);
  @$pb.TagNumber(2)
  set groupTraining(GroupTrainingRateView value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupTraining() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupTraining() => $_clearField(2);
  @$pb.TagNumber(2)
  GroupTrainingRateView ensureGroupTraining() => $_ensure(1);

  @$pb.TagNumber(3)
  PersonalTrainingRateView get personalTraining => $_getN(2);
  @$pb.TagNumber(3)
  set personalTraining(PersonalTrainingRateView value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasPersonalTraining() => $_has(2);
  @$pb.TagNumber(3)
  void clearPersonalTraining() => $_clearField(3);
  @$pb.TagNumber(3)
  PersonalTrainingRateView ensurePersonalTraining() => $_ensure(2);
}

class FixRateView extends $pb.GeneratedMessage {
  factory FixRateView({
    $fixnum.Int64? amount,
    $fixnum.Int64? nextPaymentDate,
    IntervalView? rewardInterval,
  }) {
    final result = create();
    if (amount != null) result.amount = amount;
    if (nextPaymentDate != null) result.nextPaymentDate = nextPaymentDate;
    if (rewardInterval != null) result.rewardInterval = rewardInterval;
    return result;
  }

  FixRateView._();

  factory FixRateView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory FixRateView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FixRateView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aInt64(1, _omitFieldNames ? '' : 'amount')
    ..aInt64(2, _omitFieldNames ? '' : 'nextPaymentDate')
    ..aOM<IntervalView>(3, _omitFieldNames ? '' : 'rewardInterval', subBuilder: IntervalView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FixRateView clone() => FixRateView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FixRateView copyWith(void Function(FixRateView) updates) => super.copyWith((message) => updates(message as FixRateView)) as FixRateView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FixRateView create() => FixRateView._();
  @$core.override
  FixRateView createEmptyInstance() => create();
  static $pb.PbList<FixRateView> createRepeated() => $pb.PbList<FixRateView>();
  @$core.pragma('dart2js:noInline')
  static FixRateView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FixRateView>(create);
  static FixRateView? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get amount => $_getI64(0);
  @$pb.TagNumber(1)
  set amount($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAmount() => $_has(0);
  @$pb.TagNumber(1)
  void clearAmount() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get nextPaymentDate => $_getI64(1);
  @$pb.TagNumber(2)
  set nextPaymentDate($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNextPaymentDate() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextPaymentDate() => $_clearField(2);

  @$pb.TagNumber(3)
  IntervalView get rewardInterval => $_getN(2);
  @$pb.TagNumber(3)
  set rewardInterval(IntervalView value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasRewardInterval() => $_has(2);
  @$pb.TagNumber(3)
  void clearRewardInterval() => $_clearField(3);
  @$pb.TagNumber(3)
  IntervalView ensureRewardInterval() => $_ensure(2);
}

class GroupTrainingRateView extends $pb.GeneratedMessage {
  factory GroupTrainingRateView({
    $fixnum.Int64? percent,
    $fixnum.Int64? minReward,
  }) {
    final result = create();
    if (percent != null) result.percent = percent;
    if (minReward != null) result.minReward = minReward;
    return result;
  }

  GroupTrainingRateView._();

  factory GroupTrainingRateView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory GroupTrainingRateView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupTrainingRateView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aInt64(1, _omitFieldNames ? '' : 'percent')
    ..aInt64(2, _omitFieldNames ? '' : 'minReward')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupTrainingRateView clone() => GroupTrainingRateView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupTrainingRateView copyWith(void Function(GroupTrainingRateView) updates) => super.copyWith((message) => updates(message as GroupTrainingRateView)) as GroupTrainingRateView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupTrainingRateView create() => GroupTrainingRateView._();
  @$core.override
  GroupTrainingRateView createEmptyInstance() => create();
  static $pb.PbList<GroupTrainingRateView> createRepeated() => $pb.PbList<GroupTrainingRateView>();
  @$core.pragma('dart2js:noInline')
  static GroupTrainingRateView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupTrainingRateView>(create);
  static GroupTrainingRateView? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get percent => $_getI64(0);
  @$pb.TagNumber(1)
  set percent($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPercent() => $_has(0);
  @$pb.TagNumber(1)
  void clearPercent() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get minReward => $_getI64(1);
  @$pb.TagNumber(2)
  set minReward($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMinReward() => $_has(1);
  @$pb.TagNumber(2)
  void clearMinReward() => $_clearField(2);
}

class PersonalTrainingRateView extends $pb.GeneratedMessage {
  factory PersonalTrainingRateView({
    $fixnum.Int64? percent,
  }) {
    final result = create();
    if (percent != null) result.percent = percent;
    return result;
  }

  PersonalTrainingRateView._();

  factory PersonalTrainingRateView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory PersonalTrainingRateView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'PersonalTrainingRateView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aInt64(1, _omitFieldNames ? '' : 'percent')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonalTrainingRateView clone() => PersonalTrainingRateView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonalTrainingRateView copyWith(void Function(PersonalTrainingRateView) updates) => super.copyWith((message) => updates(message as PersonalTrainingRateView)) as PersonalTrainingRateView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonalTrainingRateView create() => PersonalTrainingRateView._();
  @$core.override
  PersonalTrainingRateView createEmptyInstance() => create();
  static $pb.PbList<PersonalTrainingRateView> createRepeated() => $pb.PbList<PersonalTrainingRateView>();
  @$core.pragma('dart2js:noInline')
  static PersonalTrainingRateView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PersonalTrainingRateView>(create);
  static PersonalTrainingRateView? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get percent => $_getI64(0);
  @$pb.TagNumber(1)
  set percent($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPercent() => $_has(0);
  @$pb.TagNumber(1)
  void clearPercent() => $_clearField(1);
}

class IntervalView extends $pb.GeneratedMessage {
  factory IntervalView({
    $core.int? monthNum,
  }) {
    final result = create();
    if (monthNum != null) result.monthNum = monthNum;
    return result;
  }

  IntervalView._();

  factory IntervalView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory IntervalView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'IntervalView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'monthNum', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IntervalView clone() => IntervalView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IntervalView copyWith(void Function(IntervalView) updates) => super.copyWith((message) => updates(message as IntervalView)) as IntervalView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static IntervalView create() => IntervalView._();
  @$core.override
  IntervalView createEmptyInstance() => create();
  static $pb.PbList<IntervalView> createRepeated() => $pb.PbList<IntervalView>();
  @$core.pragma('dart2js:noInline')
  static IntervalView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<IntervalView>(create);
  static IntervalView? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get monthNum => $_getIZ(0);
  @$pb.TagNumber(1)
  set monthNum($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMonthNum() => $_has(0);
  @$pb.TagNumber(1)
  void clearMonthNum() => $_clearField(1);
}

enum StatusView_StatusView {
  notActive, 
  active, 
  notSet
}

class StatusView extends $pb.GeneratedMessage {
  factory StatusView({
    NotActive? notActive,
    ActiveStatusView? active,
  }) {
    final result = create();
    if (notActive != null) result.notActive = notActive;
    if (active != null) result.active = active;
    return result;
  }

  StatusView._();

  factory StatusView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory StatusView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, StatusView_StatusView> _StatusView_StatusViewByTag = {
    1 : StatusView_StatusView.notActive,
    2 : StatusView_StatusView.active,
    0 : StatusView_StatusView.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'StatusView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<NotActive>(1, _omitFieldNames ? '' : 'notActive', subBuilder: NotActive.create)
    ..aOM<ActiveStatusView>(2, _omitFieldNames ? '' : 'active', subBuilder: ActiveStatusView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StatusView clone() => StatusView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StatusView copyWith(void Function(StatusView) updates) => super.copyWith((message) => updates(message as StatusView)) as StatusView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StatusView create() => StatusView._();
  @$core.override
  StatusView createEmptyInstance() => create();
  static $pb.PbList<StatusView> createRepeated() => $pb.PbList<StatusView>();
  @$core.pragma('dart2js:noInline')
  static StatusView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StatusView>(create);
  static StatusView? _defaultInstance;

  StatusView_StatusView whichStatusView() => _StatusView_StatusViewByTag[$_whichOneof(0)]!;
  void clearStatusView() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  NotActive get notActive => $_getN(0);
  @$pb.TagNumber(1)
  set notActive(NotActive value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNotActive() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotActive() => $_clearField(1);
  @$pb.TagNumber(1)
  NotActive ensureNotActive() => $_ensure(0);

  @$pb.TagNumber(2)
  ActiveStatusView get active => $_getN(1);
  @$pb.TagNumber(2)
  set active(ActiveStatusView value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasActive() => $_has(1);
  @$pb.TagNumber(2)
  void clearActive() => $_clearField(2);
  @$pb.TagNumber(2)
  ActiveStatusView ensureActive() => $_ensure(1);
}

class NotActive extends $pb.GeneratedMessage {
  factory NotActive() => create();

  NotActive._();

  factory NotActive.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory NotActive.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NotActive', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotActive clone() => NotActive()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotActive copyWith(void Function(NotActive) updates) => super.copyWith((message) => updates(message as NotActive)) as NotActive;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NotActive create() => NotActive._();
  @$core.override
  NotActive createEmptyInstance() => create();
  static $pb.PbList<NotActive> createRepeated() => $pb.PbList<NotActive>();
  @$core.pragma('dart2js:noInline')
  static NotActive getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NotActive>(create);
  static NotActive? _defaultInstance;
}

class ActiveStatusView extends $pb.GeneratedMessage {
  factory ActiveStatusView({
    $fixnum.Int64? startDate,
    $fixnum.Int64? endDate,
  }) {
    final result = create();
    if (startDate != null) result.startDate = startDate;
    if (endDate != null) result.endDate = endDate;
    return result;
  }

  ActiveStatusView._();

  factory ActiveStatusView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory ActiveStatusView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ActiveStatusView', package: const $pb.PackageName(_omitMessageNames ? '' : 'user'), createEmptyInstance: create)
    ..aInt64(1, _omitFieldNames ? '' : 'startDate')
    ..aInt64(2, _omitFieldNames ? '' : 'endDate')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ActiveStatusView clone() => ActiveStatusView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ActiveStatusView copyWith(void Function(ActiveStatusView) updates) => super.copyWith((message) => updates(message as ActiveStatusView)) as ActiveStatusView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ActiveStatusView create() => ActiveStatusView._();
  @$core.override
  ActiveStatusView createEmptyInstance() => create();
  static $pb.PbList<ActiveStatusView> createRepeated() => $pb.PbList<ActiveStatusView>();
  @$core.pragma('dart2js:noInline')
  static ActiveStatusView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ActiveStatusView>(create);
  static ActiveStatusView? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get startDate => $_getI64(0);
  @$pb.TagNumber(1)
  set startDate($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStartDate() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartDate() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get endDate => $_getI64(1);
  @$pb.TagNumber(2)
  set endDate($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEndDate() => $_has(1);
  @$pb.TagNumber(2)
  void clearEndDate() => $_clearField(2);
}


const $core.bool _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
