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

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'id.pb.dart' as $4;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class LocationView extends $pb.GeneratedMessage {
  factory LocationView({
    $4.ObjectId? id,
    $core.String? name,
    $core.String? address,
    WorkingHoursView? workingHours,
    $core.Iterable<HallView>? halls,
    $fixnum.Int64? version,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (address != null) result.address = address;
    if (workingHours != null) result.workingHours = workingHours;
    if (halls != null) result.halls.addAll(halls);
    if (version != null) result.version = version;
    return result;
  }

  LocationView._();

  factory LocationView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory LocationView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'LocationView', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $4.ObjectId.create)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'address')
    ..aOM<WorkingHoursView>(4, _omitFieldNames ? '' : 'workingHours', subBuilder: WorkingHoursView.create)
    ..pc<HallView>(5, _omitFieldNames ? '' : 'halls', $pb.PbFieldType.PM, subBuilder: HallView.create)
    ..a<$fixnum.Int64>(6, _omitFieldNames ? '' : 'version', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationView clone() => LocationView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationView copyWith(void Function(LocationView) updates) => super.copyWith((message) => updates(message as LocationView)) as LocationView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LocationView create() => LocationView._();
  @$core.override
  LocationView createEmptyInstance() => create();
  static $pb.PbList<LocationView> createRepeated() => $pb.PbList<LocationView>();
  @$core.pragma('dart2js:noInline')
  static LocationView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LocationView>(create);
  static LocationView? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureId() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get address => $_getSZ(2);
  @$pb.TagNumber(3)
  set address($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAddress() => $_has(2);
  @$pb.TagNumber(3)
  void clearAddress() => $_clearField(3);

  @$pb.TagNumber(4)
  WorkingHoursView get workingHours => $_getN(3);
  @$pb.TagNumber(4)
  set workingHours(WorkingHoursView value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasWorkingHours() => $_has(3);
  @$pb.TagNumber(4)
  void clearWorkingHours() => $_clearField(4);
  @$pb.TagNumber(4)
  WorkingHoursView ensureWorkingHours() => $_ensure(3);

  @$pb.TagNumber(5)
  $pb.PbList<HallView> get halls => $_getList(4);

  @$pb.TagNumber(6)
  $fixnum.Int64 get version => $_getI64(5);
  @$pb.TagNumber(6)
  set version($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasVersion() => $_has(5);
  @$pb.TagNumber(6)
  void clearVersion() => $_clearField(6);
}

class WorkingHoursView extends $pb.GeneratedMessage {
  factory WorkingHoursView({
    DayHoursView? monday,
    DayHoursView? tuesday,
    DayHoursView? wednesday,
    DayHoursView? thursday,
    DayHoursView? friday,
    DayHoursView? saturday,
    DayHoursView? sunday,
  }) {
    final result = create();
    if (monday != null) result.monday = monday;
    if (tuesday != null) result.tuesday = tuesday;
    if (wednesday != null) result.wednesday = wednesday;
    if (thursday != null) result.thursday = thursday;
    if (friday != null) result.friday = friday;
    if (saturday != null) result.saturday = saturday;
    if (sunday != null) result.sunday = sunday;
    return result;
  }

  WorkingHoursView._();

  factory WorkingHoursView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory WorkingHoursView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'WorkingHoursView', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<DayHoursView>(1, _omitFieldNames ? '' : 'monday', subBuilder: DayHoursView.create)
    ..aOM<DayHoursView>(2, _omitFieldNames ? '' : 'tuesday', subBuilder: DayHoursView.create)
    ..aOM<DayHoursView>(3, _omitFieldNames ? '' : 'wednesday', subBuilder: DayHoursView.create)
    ..aOM<DayHoursView>(4, _omitFieldNames ? '' : 'thursday', subBuilder: DayHoursView.create)
    ..aOM<DayHoursView>(5, _omitFieldNames ? '' : 'friday', subBuilder: DayHoursView.create)
    ..aOM<DayHoursView>(6, _omitFieldNames ? '' : 'saturday', subBuilder: DayHoursView.create)
    ..aOM<DayHoursView>(7, _omitFieldNames ? '' : 'sunday', subBuilder: DayHoursView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkingHoursView clone() => WorkingHoursView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkingHoursView copyWith(void Function(WorkingHoursView) updates) => super.copyWith((message) => updates(message as WorkingHoursView)) as WorkingHoursView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static WorkingHoursView create() => WorkingHoursView._();
  @$core.override
  WorkingHoursView createEmptyInstance() => create();
  static $pb.PbList<WorkingHoursView> createRepeated() => $pb.PbList<WorkingHoursView>();
  @$core.pragma('dart2js:noInline')
  static WorkingHoursView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<WorkingHoursView>(create);
  static WorkingHoursView? _defaultInstance;

  @$pb.TagNumber(1)
  DayHoursView get monday => $_getN(0);
  @$pb.TagNumber(1)
  set monday(DayHoursView value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMonday() => $_has(0);
  @$pb.TagNumber(1)
  void clearMonday() => $_clearField(1);
  @$pb.TagNumber(1)
  DayHoursView ensureMonday() => $_ensure(0);

  @$pb.TagNumber(2)
  DayHoursView get tuesday => $_getN(1);
  @$pb.TagNumber(2)
  set tuesday(DayHoursView value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasTuesday() => $_has(1);
  @$pb.TagNumber(2)
  void clearTuesday() => $_clearField(2);
  @$pb.TagNumber(2)
  DayHoursView ensureTuesday() => $_ensure(1);

  @$pb.TagNumber(3)
  DayHoursView get wednesday => $_getN(2);
  @$pb.TagNumber(3)
  set wednesday(DayHoursView value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasWednesday() => $_has(2);
  @$pb.TagNumber(3)
  void clearWednesday() => $_clearField(3);
  @$pb.TagNumber(3)
  DayHoursView ensureWednesday() => $_ensure(2);

  @$pb.TagNumber(4)
  DayHoursView get thursday => $_getN(3);
  @$pb.TagNumber(4)
  set thursday(DayHoursView value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasThursday() => $_has(3);
  @$pb.TagNumber(4)
  void clearThursday() => $_clearField(4);
  @$pb.TagNumber(4)
  DayHoursView ensureThursday() => $_ensure(3);

  @$pb.TagNumber(5)
  DayHoursView get friday => $_getN(4);
  @$pb.TagNumber(5)
  set friday(DayHoursView value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasFriday() => $_has(4);
  @$pb.TagNumber(5)
  void clearFriday() => $_clearField(5);
  @$pb.TagNumber(5)
  DayHoursView ensureFriday() => $_ensure(4);

  @$pb.TagNumber(6)
  DayHoursView get saturday => $_getN(5);
  @$pb.TagNumber(6)
  set saturday(DayHoursView value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasSaturday() => $_has(5);
  @$pb.TagNumber(6)
  void clearSaturday() => $_clearField(6);
  @$pb.TagNumber(6)
  DayHoursView ensureSaturday() => $_ensure(5);

  @$pb.TagNumber(7)
  DayHoursView get sunday => $_getN(6);
  @$pb.TagNumber(7)
  set sunday(DayHoursView value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasSunday() => $_has(6);
  @$pb.TagNumber(7)
  void clearSunday() => $_clearField(7);
  @$pb.TagNumber(7)
  DayHoursView ensureSunday() => $_ensure(6);
}

class DayHoursView extends $pb.GeneratedMessage {
  factory DayHoursView({
    $fixnum.Int64? open,
    $fixnum.Int64? close,
  }) {
    final result = create();
    if (open != null) result.open = open;
    if (close != null) result.close = close;
    return result;
  }

  DayHoursView._();

  factory DayHoursView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory DayHoursView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'DayHoursView', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aInt64(1, _omitFieldNames ? '' : 'open')
    ..aInt64(2, _omitFieldNames ? '' : 'close')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DayHoursView clone() => DayHoursView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DayHoursView copyWith(void Function(DayHoursView) updates) => super.copyWith((message) => updates(message as DayHoursView)) as DayHoursView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DayHoursView create() => DayHoursView._();
  @$core.override
  DayHoursView createEmptyInstance() => create();
  static $pb.PbList<DayHoursView> createRepeated() => $pb.PbList<DayHoursView>();
  @$core.pragma('dart2js:noInline')
  static DayHoursView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DayHoursView>(create);
  static DayHoursView? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get open => $_getI64(0);
  @$pb.TagNumber(1)
  set open($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOpen() => $_has(0);
  @$pb.TagNumber(1)
  void clearOpen() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get close => $_getI64(1);
  @$pb.TagNumber(2)
  set close($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasClose() => $_has(1);
  @$pb.TagNumber(2)
  void clearClose() => $_clearField(2);
}

class HallView extends $pb.GeneratedMessage {
  factory HallView({
    $4.ObjectId? id,
    $core.String? name,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    return result;
  }

  HallView._();

  factory HallView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory HallView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'HallView', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $4.ObjectId.create)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  HallView clone() => HallView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  HallView copyWith(void Function(HallView) updates) => super.copyWith((message) => updates(message as HallView)) as HallView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static HallView create() => HallView._();
  @$core.override
  HallView createEmptyInstance() => create();
  static $pb.PbList<HallView> createRepeated() => $pb.PbList<HallView>();
  @$core.pragma('dart2js:noInline')
  static HallView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<HallView>(create);
  static HallView? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureId() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);
}

/// Request/Response messages
class LocationRequest extends $pb.GeneratedMessage {
  factory LocationRequest({
    $4.ObjectId? id,
  }) {
    final result = create();
    if (id != null) result.id = id;
    return result;
  }

  LocationRequest._();

  factory LocationRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory LocationRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'LocationRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationRequest clone() => LocationRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationRequest copyWith(void Function(LocationRequest) updates) => super.copyWith((message) => updates(message as LocationRequest)) as LocationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LocationRequest create() => LocationRequest._();
  @$core.override
  LocationRequest createEmptyInstance() => create();
  static $pb.PbList<LocationRequest> createRepeated() => $pb.PbList<LocationRequest>();
  @$core.pragma('dart2js:noInline')
  static LocationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LocationRequest>(create);
  static LocationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureId() => $_ensure(0);
}

class LocationListRequest extends $pb.GeneratedMessage {
  factory LocationListRequest() => create();

  LocationListRequest._();

  factory LocationListRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory LocationListRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'LocationListRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationListRequest clone() => LocationListRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationListRequest copyWith(void Function(LocationListRequest) updates) => super.copyWith((message) => updates(message as LocationListRequest)) as LocationListRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LocationListRequest create() => LocationListRequest._();
  @$core.override
  LocationListRequest createEmptyInstance() => create();
  static $pb.PbList<LocationListRequest> createRepeated() => $pb.PbList<LocationListRequest>();
  @$core.pragma('dart2js:noInline')
  static LocationListRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LocationListRequest>(create);
  static LocationListRequest? _defaultInstance;
}

class LocationListView extends $pb.GeneratedMessage {
  factory LocationListView({
    $core.Iterable<LocationView>? locations,
  }) {
    final result = create();
    if (locations != null) result.locations.addAll(locations);
    return result;
  }

  LocationListView._();

  factory LocationListView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory LocationListView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'LocationListView', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..pc<LocationView>(1, _omitFieldNames ? '' : 'locations', $pb.PbFieldType.PM, subBuilder: LocationView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationListView clone() => LocationListView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LocationListView copyWith(void Function(LocationListView) updates) => super.copyWith((message) => updates(message as LocationListView)) as LocationListView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LocationListView create() => LocationListView._();
  @$core.override
  LocationListView createEmptyInstance() => create();
  static $pb.PbList<LocationListView> createRepeated() => $pb.PbList<LocationListView>();
  @$core.pragma('dart2js:noInline')
  static LocationListView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LocationListView>(create);
  static LocationListView? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<LocationView> get locations => $_getList(0);
}

class CreateLocationRequest extends $pb.GeneratedMessage {
  factory CreateLocationRequest({
    $core.String? name,
    $core.String? address,
    WorkingHoursView? workingHours,
  }) {
    final result = create();
    if (name != null) result.name = name;
    if (address != null) result.address = address;
    if (workingHours != null) result.workingHours = workingHours;
    return result;
  }

  CreateLocationRequest._();

  factory CreateLocationRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory CreateLocationRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateLocationRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'address')
    ..aOM<WorkingHoursView>(3, _omitFieldNames ? '' : 'workingHours', subBuilder: WorkingHoursView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateLocationRequest clone() => CreateLocationRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateLocationRequest copyWith(void Function(CreateLocationRequest) updates) => super.copyWith((message) => updates(message as CreateLocationRequest)) as CreateLocationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateLocationRequest create() => CreateLocationRequest._();
  @$core.override
  CreateLocationRequest createEmptyInstance() => create();
  static $pb.PbList<CreateLocationRequest> createRepeated() => $pb.PbList<CreateLocationRequest>();
  @$core.pragma('dart2js:noInline')
  static CreateLocationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateLocationRequest>(create);
  static CreateLocationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get address => $_getSZ(1);
  @$pb.TagNumber(2)
  set address($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAddress() => $_has(1);
  @$pb.TagNumber(2)
  void clearAddress() => $_clearField(2);

  @$pb.TagNumber(3)
  WorkingHoursView get workingHours => $_getN(2);
  @$pb.TagNumber(3)
  set workingHours(WorkingHoursView value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasWorkingHours() => $_has(2);
  @$pb.TagNumber(3)
  void clearWorkingHours() => $_clearField(3);
  @$pb.TagNumber(3)
  WorkingHoursView ensureWorkingHours() => $_ensure(2);
}

class CreateLocationResponse extends $pb.GeneratedMessage {
  factory CreateLocationResponse({
    $4.ObjectId? id,
  }) {
    final result = create();
    if (id != null) result.id = id;
    return result;
  }

  CreateLocationResponse._();

  factory CreateLocationResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory CreateLocationResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateLocationResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateLocationResponse clone() => CreateLocationResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateLocationResponse copyWith(void Function(CreateLocationResponse) updates) => super.copyWith((message) => updates(message as CreateLocationResponse)) as CreateLocationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateLocationResponse create() => CreateLocationResponse._();
  @$core.override
  CreateLocationResponse createEmptyInstance() => create();
  static $pb.PbList<CreateLocationResponse> createRepeated() => $pb.PbList<CreateLocationResponse>();
  @$core.pragma('dart2js:noInline')
  static CreateLocationResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateLocationResponse>(create);
  static CreateLocationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureId() => $_ensure(0);
}

class UpdateLocationRequest extends $pb.GeneratedMessage {
  factory UpdateLocationRequest({
    $4.ObjectId? id,
    $core.String? name,
    $core.String? address,
    WorkingHoursView? workingHours,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (address != null) result.address = address;
    if (workingHours != null) result.workingHours = workingHours;
    return result;
  }

  UpdateLocationRequest._();

  factory UpdateLocationRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UpdateLocationRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateLocationRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $4.ObjectId.create)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'address')
    ..aOM<WorkingHoursView>(4, _omitFieldNames ? '' : 'workingHours', subBuilder: WorkingHoursView.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateLocationRequest clone() => UpdateLocationRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateLocationRequest copyWith(void Function(UpdateLocationRequest) updates) => super.copyWith((message) => updates(message as UpdateLocationRequest)) as UpdateLocationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateLocationRequest create() => UpdateLocationRequest._();
  @$core.override
  UpdateLocationRequest createEmptyInstance() => create();
  static $pb.PbList<UpdateLocationRequest> createRepeated() => $pb.PbList<UpdateLocationRequest>();
  @$core.pragma('dart2js:noInline')
  static UpdateLocationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateLocationRequest>(create);
  static UpdateLocationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureId() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get address => $_getSZ(2);
  @$pb.TagNumber(3)
  set address($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAddress() => $_has(2);
  @$pb.TagNumber(3)
  void clearAddress() => $_clearField(3);

  @$pb.TagNumber(4)
  WorkingHoursView get workingHours => $_getN(3);
  @$pb.TagNumber(4)
  set workingHours(WorkingHoursView value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasWorkingHours() => $_has(3);
  @$pb.TagNumber(4)
  void clearWorkingHours() => $_clearField(4);
  @$pb.TagNumber(4)
  WorkingHoursView ensureWorkingHours() => $_ensure(3);
}

class UpdateLocationResponse extends $pb.GeneratedMessage {
  factory UpdateLocationResponse() => create();

  UpdateLocationResponse._();

  factory UpdateLocationResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UpdateLocationResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateLocationResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateLocationResponse clone() => UpdateLocationResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateLocationResponse copyWith(void Function(UpdateLocationResponse) updates) => super.copyWith((message) => updates(message as UpdateLocationResponse)) as UpdateLocationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateLocationResponse create() => UpdateLocationResponse._();
  @$core.override
  UpdateLocationResponse createEmptyInstance() => create();
  static $pb.PbList<UpdateLocationResponse> createRepeated() => $pb.PbList<UpdateLocationResponse>();
  @$core.pragma('dart2js:noInline')
  static UpdateLocationResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateLocationResponse>(create);
  static UpdateLocationResponse? _defaultInstance;
}

class DeleteLocationRequest extends $pb.GeneratedMessage {
  factory DeleteLocationRequest({
    $4.ObjectId? id,
  }) {
    final result = create();
    if (id != null) result.id = id;
    return result;
  }

  DeleteLocationRequest._();

  factory DeleteLocationRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory DeleteLocationRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'DeleteLocationRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'id', subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteLocationRequest clone() => DeleteLocationRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteLocationRequest copyWith(void Function(DeleteLocationRequest) updates) => super.copyWith((message) => updates(message as DeleteLocationRequest)) as DeleteLocationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeleteLocationRequest create() => DeleteLocationRequest._();
  @$core.override
  DeleteLocationRequest createEmptyInstance() => create();
  static $pb.PbList<DeleteLocationRequest> createRepeated() => $pb.PbList<DeleteLocationRequest>();
  @$core.pragma('dart2js:noInline')
  static DeleteLocationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeleteLocationRequest>(create);
  static DeleteLocationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureId() => $_ensure(0);
}

class DeleteLocationResponse extends $pb.GeneratedMessage {
  factory DeleteLocationResponse() => create();

  DeleteLocationResponse._();

  factory DeleteLocationResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory DeleteLocationResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'DeleteLocationResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteLocationResponse clone() => DeleteLocationResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteLocationResponse copyWith(void Function(DeleteLocationResponse) updates) => super.copyWith((message) => updates(message as DeleteLocationResponse)) as DeleteLocationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeleteLocationResponse create() => DeleteLocationResponse._();
  @$core.override
  DeleteLocationResponse createEmptyInstance() => create();
  static $pb.PbList<DeleteLocationResponse> createRepeated() => $pb.PbList<DeleteLocationResponse>();
  @$core.pragma('dart2js:noInline')
  static DeleteLocationResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeleteLocationResponse>(create);
  static DeleteLocationResponse? _defaultInstance;
}

class AddHallRequest extends $pb.GeneratedMessage {
  factory AddHallRequest({
    $4.ObjectId? locationId,
    $core.String? hallName,
  }) {
    final result = create();
    if (locationId != null) result.locationId = locationId;
    if (hallName != null) result.hallName = hallName;
    return result;
  }

  AddHallRequest._();

  factory AddHallRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory AddHallRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddHallRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'locationId', subBuilder: $4.ObjectId.create)
    ..aOS(2, _omitFieldNames ? '' : 'hallName')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddHallRequest clone() => AddHallRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddHallRequest copyWith(void Function(AddHallRequest) updates) => super.copyWith((message) => updates(message as AddHallRequest)) as AddHallRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddHallRequest create() => AddHallRequest._();
  @$core.override
  AddHallRequest createEmptyInstance() => create();
  static $pb.PbList<AddHallRequest> createRepeated() => $pb.PbList<AddHallRequest>();
  @$core.pragma('dart2js:noInline')
  static AddHallRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddHallRequest>(create);
  static AddHallRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get locationId => $_getN(0);
  @$pb.TagNumber(1)
  set locationId($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasLocationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLocationId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureLocationId() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get hallName => $_getSZ(1);
  @$pb.TagNumber(2)
  set hallName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHallName() => $_has(1);
  @$pb.TagNumber(2)
  void clearHallName() => $_clearField(2);
}

class AddHallResponse extends $pb.GeneratedMessage {
  factory AddHallResponse({
    $4.ObjectId? hallId,
  }) {
    final result = create();
    if (hallId != null) result.hallId = hallId;
    return result;
  }

  AddHallResponse._();

  factory AddHallResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory AddHallResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddHallResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'hallId', subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddHallResponse clone() => AddHallResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddHallResponse copyWith(void Function(AddHallResponse) updates) => super.copyWith((message) => updates(message as AddHallResponse)) as AddHallResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddHallResponse create() => AddHallResponse._();
  @$core.override
  AddHallResponse createEmptyInstance() => create();
  static $pb.PbList<AddHallResponse> createRepeated() => $pb.PbList<AddHallResponse>();
  @$core.pragma('dart2js:noInline')
  static AddHallResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddHallResponse>(create);
  static AddHallResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get hallId => $_getN(0);
  @$pb.TagNumber(1)
  set hallId($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHallId() => $_has(0);
  @$pb.TagNumber(1)
  void clearHallId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureHallId() => $_ensure(0);
}

class RemoveHallRequest extends $pb.GeneratedMessage {
  factory RemoveHallRequest({
    $4.ObjectId? locationId,
    $4.ObjectId? hallId,
  }) {
    final result = create();
    if (locationId != null) result.locationId = locationId;
    if (hallId != null) result.hallId = hallId;
    return result;
  }

  RemoveHallRequest._();

  factory RemoveHallRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory RemoveHallRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveHallRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'locationId', subBuilder: $4.ObjectId.create)
    ..aOM<$4.ObjectId>(2, _omitFieldNames ? '' : 'hallId', subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveHallRequest clone() => RemoveHallRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveHallRequest copyWith(void Function(RemoveHallRequest) updates) => super.copyWith((message) => updates(message as RemoveHallRequest)) as RemoveHallRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveHallRequest create() => RemoveHallRequest._();
  @$core.override
  RemoveHallRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveHallRequest> createRepeated() => $pb.PbList<RemoveHallRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveHallRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveHallRequest>(create);
  static RemoveHallRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get locationId => $_getN(0);
  @$pb.TagNumber(1)
  set locationId($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasLocationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLocationId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureLocationId() => $_ensure(0);

  @$pb.TagNumber(2)
  $4.ObjectId get hallId => $_getN(1);
  @$pb.TagNumber(2)
  set hallId($4.ObjectId value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasHallId() => $_has(1);
  @$pb.TagNumber(2)
  void clearHallId() => $_clearField(2);
  @$pb.TagNumber(2)
  $4.ObjectId ensureHallId() => $_ensure(1);
}

class RemoveHallResponse extends $pb.GeneratedMessage {
  factory RemoveHallResponse() => create();

  RemoveHallResponse._();

  factory RemoveHallResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory RemoveHallResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveHallResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveHallResponse clone() => RemoveHallResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveHallResponse copyWith(void Function(RemoveHallResponse) updates) => super.copyWith((message) => updates(message as RemoveHallResponse)) as RemoveHallResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveHallResponse create() => RemoveHallResponse._();
  @$core.override
  RemoveHallResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveHallResponse> createRepeated() => $pb.PbList<RemoveHallResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveHallResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveHallResponse>(create);
  static RemoveHallResponse? _defaultInstance;
}

class UpdateHallRequest extends $pb.GeneratedMessage {
  factory UpdateHallRequest({
    $4.ObjectId? locationId,
    $4.ObjectId? hallId,
    $core.String? name,
  }) {
    final result = create();
    if (locationId != null) result.locationId = locationId;
    if (hallId != null) result.hallId = hallId;
    if (name != null) result.name = name;
    return result;
  }

  UpdateHallRequest._();

  factory UpdateHallRequest.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UpdateHallRequest.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateHallRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'locationId', subBuilder: $4.ObjectId.create)
    ..aOM<$4.ObjectId>(2, _omitFieldNames ? '' : 'hallId', subBuilder: $4.ObjectId.create)
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateHallRequest clone() => UpdateHallRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateHallRequest copyWith(void Function(UpdateHallRequest) updates) => super.copyWith((message) => updates(message as UpdateHallRequest)) as UpdateHallRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateHallRequest create() => UpdateHallRequest._();
  @$core.override
  UpdateHallRequest createEmptyInstance() => create();
  static $pb.PbList<UpdateHallRequest> createRepeated() => $pb.PbList<UpdateHallRequest>();
  @$core.pragma('dart2js:noInline')
  static UpdateHallRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateHallRequest>(create);
  static UpdateHallRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get locationId => $_getN(0);
  @$pb.TagNumber(1)
  set locationId($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasLocationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLocationId() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureLocationId() => $_ensure(0);

  @$pb.TagNumber(2)
  $4.ObjectId get hallId => $_getN(1);
  @$pb.TagNumber(2)
  set hallId($4.ObjectId value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasHallId() => $_has(1);
  @$pb.TagNumber(2)
  void clearHallId() => $_clearField(2);
  @$pb.TagNumber(2)
  $4.ObjectId ensureHallId() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);
}

class UpdateHallResponse extends $pb.GeneratedMessage {
  factory UpdateHallResponse() => create();

  UpdateHallResponse._();

  factory UpdateHallResponse.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory UpdateHallResponse.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateHallResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'locations'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateHallResponse clone() => UpdateHallResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateHallResponse copyWith(void Function(UpdateHallResponse) updates) => super.copyWith((message) => updates(message as UpdateHallResponse)) as UpdateHallResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateHallResponse create() => UpdateHallResponse._();
  @$core.override
  UpdateHallResponse createEmptyInstance() => create();
  static $pb.PbList<UpdateHallResponse> createRepeated() => $pb.PbList<UpdateHallResponse>();
  @$core.pragma('dart2js:noInline')
  static UpdateHallResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateHallResponse>(create);
  static UpdateHallResponse? _defaultInstance;
}


const $core.bool _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
