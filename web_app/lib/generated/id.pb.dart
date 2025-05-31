//
//  Generated code. Do not modify.
//  source: id.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ObjectId extends $pb.GeneratedMessage {
  factory ObjectId({
    $core.List<$core.int>? value,
  }) {
    final result = create();
    if (value != null) result.value = value;
    return result;
  }

  ObjectId._();

  factory ObjectId.fromBuffer($core.List<$core.int> data, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(data, registry);
  factory ObjectId.fromJson($core.String json, [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ObjectId', package: const $pb.PackageName(_omitMessageNames ? '' : 'id'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'value', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ObjectId clone() => ObjectId()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ObjectId copyWith(void Function(ObjectId) updates) => super.copyWith((message) => updates(message as ObjectId)) as ObjectId;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ObjectId create() => ObjectId._();
  @$core.override
  ObjectId createEmptyInstance() => create();
  static $pb.PbList<ObjectId> createRepeated() => $pb.PbList<ObjectId>();
  @$core.pragma('dart2js:noInline')
  static ObjectId getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObjectId>(create);
  static ObjectId? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get value => $_getN(0);
  @$pb.TagNumber(1)
  set value($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearValue() => $_clearField(1);
}


const $core.bool _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
