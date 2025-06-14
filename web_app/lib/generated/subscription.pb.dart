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

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'id.pb.dart' as $4;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum SubscriptionTypeView_SubscriptionType {
  group, 
  personal, 
  notSet
}

class SubscriptionTypeView extends $pb.GeneratedMessage {
  factory SubscriptionTypeView({
    Group? group,
    Personal? personal,
  }) {
    final result = create();
    if (group != null) result.group = group;
    if (personal != null) result.personal = personal;
    return result;
  }

  SubscriptionTypeView._();

  factory SubscriptionTypeView.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory SubscriptionTypeView.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, SubscriptionTypeView_SubscriptionType> _SubscriptionTypeView_SubscriptionTypeByTag = {
    1 : SubscriptionTypeView_SubscriptionType.group,
    2 : SubscriptionTypeView_SubscriptionType.personal,
    0 : SubscriptionTypeView_SubscriptionType.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SubscriptionTypeView', package: const $pb.PackageName(_omitMessageNames ? '' : 'subscription'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<Group>(1, _omitFieldNames ? '' : 'group', subBuilder: Group.create)
    ..aOM<Personal>(2, _omitFieldNames ? '' : 'personal', subBuilder: Personal.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SubscriptionTypeView clone() => SubscriptionTypeView()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SubscriptionTypeView copyWith(void Function(SubscriptionTypeView) updates) => super.copyWith((message) => updates(message as SubscriptionTypeView)) as SubscriptionTypeView;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SubscriptionTypeView create() => SubscriptionTypeView._();
  @$core.override
  SubscriptionTypeView createEmptyInstance() => create();
  static $pb.PbList<SubscriptionTypeView> createRepeated() => $pb.PbList<SubscriptionTypeView>();
  @$core.pragma('dart2js:noInline')
  static SubscriptionTypeView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SubscriptionTypeView>(create);
  static SubscriptionTypeView? _defaultInstance;

  SubscriptionTypeView_SubscriptionType whichSubscriptionType() => _SubscriptionTypeView_SubscriptionTypeByTag[$_whichOneof(0)]!;
  void clearSubscriptionType() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Group get group => $_getN(0);
  @$pb.TagNumber(1)
  set group(Group value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGroup() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroup() => $_clearField(1);
  @$pb.TagNumber(1)
  Group ensureGroup() => $_ensure(0);

  @$pb.TagNumber(2)
  Personal get personal => $_getN(1);
  @$pb.TagNumber(2)
  set personal(Personal value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonal() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonal() => $_clearField(2);
  @$pb.TagNumber(2)
  Personal ensurePersonal() => $_ensure(1);
}

class Group extends $pb.GeneratedMessage {
  factory Group({
    $core.Iterable<$4.ObjectId>? programFilter,
  }) {
    final result = create();
    if (programFilter != null) result.programFilter.addAll(programFilter);
    return result;
  }

  Group._();

  factory Group.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory Group.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Group', package: const $pb.PackageName(_omitMessageNames ? '' : 'subscription'), createEmptyInstance: create)
    ..pc<$4.ObjectId>(1, _omitFieldNames ? '' : 'programFilter', $pb.PbFieldType.PM, subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Group clone() => Group()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Group copyWith(void Function(Group) updates) => super.copyWith((message) => updates(message as Group)) as Group;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Group create() => Group._();
  @$core.override
  Group createEmptyInstance() => create();
  static $pb.PbList<Group> createRepeated() => $pb.PbList<Group>();
  @$core.pragma('dart2js:noInline')
  static Group getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Group>(create);
  static Group? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$4.ObjectId> get programFilter => $_getList(0);
}

class Personal extends $pb.GeneratedMessage {
  factory Personal({
    $4.ObjectId? couchFilter,
  }) {
    final result = create();
    if (couchFilter != null) result.couchFilter = couchFilter;
    return result;
  }

  Personal._();

  factory Personal.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory Personal.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Personal', package: const $pb.PackageName(_omitMessageNames ? '' : 'subscription'), createEmptyInstance: create)
    ..aOM<$4.ObjectId>(1, _omitFieldNames ? '' : 'couchFilter', subBuilder: $4.ObjectId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Personal clone() => Personal()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Personal copyWith(void Function(Personal) updates) => super.copyWith((message) => updates(message as Personal)) as Personal;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Personal create() => Personal._();
  @$core.override
  Personal createEmptyInstance() => create();
  static $pb.PbList<Personal> createRepeated() => $pb.PbList<Personal>();
  @$core.pragma('dart2js:noInline')
  static Personal getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Personal>(create);
  static Personal? _defaultInstance;

  @$pb.TagNumber(1)
  $4.ObjectId get couchFilter => $_getN(0);
  @$pb.TagNumber(1)
  set couchFilter($4.ObjectId value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCouchFilter() => $_has(0);
  @$pb.TagNumber(1)
  void clearCouchFilter() => $_clearField(1);
  @$pb.TagNumber(1)
  $4.ObjectId ensureCouchFilter() => $_ensure(0);
}


const $core.bool _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
