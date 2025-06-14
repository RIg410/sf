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

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'locations.pb.dart' as $1;

export 'locations.pb.dart';

@$pb.GrpcServiceName('locations.LocationsService')
class LocationsServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  static final _$get = $grpc.ClientMethod<$1.LocationRequest, $1.LocationView>(
      '/locations.LocationsService/get',
      ($1.LocationRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.LocationView.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$1.LocationListRequest, $1.LocationListView>(
      '/locations.LocationsService/list',
      ($1.LocationListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.LocationListView.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$1.CreateLocationRequest, $1.CreateLocationResponse>(
      '/locations.LocationsService/create',
      ($1.CreateLocationRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.CreateLocationResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$1.UpdateLocationRequest, $1.UpdateLocationResponse>(
      '/locations.LocationsService/update',
      ($1.UpdateLocationRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.UpdateLocationResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$1.DeleteLocationRequest, $1.DeleteLocationResponse>(
      '/locations.LocationsService/delete',
      ($1.DeleteLocationRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.DeleteLocationResponse.fromBuffer(value));
  static final _$add_hall = $grpc.ClientMethod<$1.AddHallRequest, $1.AddHallResponse>(
      '/locations.LocationsService/add_hall',
      ($1.AddHallRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.AddHallResponse.fromBuffer(value));
  static final _$remove_hall = $grpc.ClientMethod<$1.RemoveHallRequest, $1.RemoveHallResponse>(
      '/locations.LocationsService/remove_hall',
      ($1.RemoveHallRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.RemoveHallResponse.fromBuffer(value));
  static final _$update_hall = $grpc.ClientMethod<$1.UpdateHallRequest, $1.UpdateHallResponse>(
      '/locations.LocationsService/update_hall',
      ($1.UpdateHallRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.UpdateHallResponse.fromBuffer(value));

  LocationsServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$1.LocationView> get($1.LocationRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseFuture<$1.LocationListView> list($1.LocationListRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$list, request, options: options);
  }

  $grpc.ResponseFuture<$1.CreateLocationResponse> create($1.CreateLocationRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$1.UpdateLocationResponse> update($1.UpdateLocationRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$1.DeleteLocationResponse> delete($1.DeleteLocationRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }

  $grpc.ResponseFuture<$1.AddHallResponse> add_hall($1.AddHallRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$add_hall, request, options: options);
  }

  $grpc.ResponseFuture<$1.RemoveHallResponse> remove_hall($1.RemoveHallRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$remove_hall, request, options: options);
  }

  $grpc.ResponseFuture<$1.UpdateHallResponse> update_hall($1.UpdateHallRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update_hall, request, options: options);
  }
}

@$pb.GrpcServiceName('locations.LocationsService')
abstract class LocationsServiceBase extends $grpc.Service {
  $core.String get $name => 'locations.LocationsService';

  LocationsServiceBase() {
    $addMethod($grpc.ServiceMethod<$1.LocationRequest, $1.LocationView>(
        'get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.LocationRequest.fromBuffer(value),
        ($1.LocationView value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.LocationListRequest, $1.LocationListView>(
        'list',
        list_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.LocationListRequest.fromBuffer(value),
        ($1.LocationListView value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.CreateLocationRequest, $1.CreateLocationResponse>(
        'create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.CreateLocationRequest.fromBuffer(value),
        ($1.CreateLocationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.UpdateLocationRequest, $1.UpdateLocationResponse>(
        'update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.UpdateLocationRequest.fromBuffer(value),
        ($1.UpdateLocationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.DeleteLocationRequest, $1.DeleteLocationResponse>(
        'delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.DeleteLocationRequest.fromBuffer(value),
        ($1.DeleteLocationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.AddHallRequest, $1.AddHallResponse>(
        'add_hall',
        add_hall_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.AddHallRequest.fromBuffer(value),
        ($1.AddHallResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.RemoveHallRequest, $1.RemoveHallResponse>(
        'remove_hall',
        remove_hall_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.RemoveHallRequest.fromBuffer(value),
        ($1.RemoveHallResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.UpdateHallRequest, $1.UpdateHallResponse>(
        'update_hall',
        update_hall_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.UpdateHallRequest.fromBuffer(value),
        ($1.UpdateHallResponse value) => value.writeToBuffer()));
  }

  $async.Future<$1.LocationView> get_Pre($grpc.ServiceCall $call, $async.Future<$1.LocationRequest> $request) async {
    return get($call, await $request);
  }

  $async.Future<$1.LocationListView> list_Pre($grpc.ServiceCall $call, $async.Future<$1.LocationListRequest> $request) async {
    return list($call, await $request);
  }

  $async.Future<$1.CreateLocationResponse> create_Pre($grpc.ServiceCall $call, $async.Future<$1.CreateLocationRequest> $request) async {
    return create($call, await $request);
  }

  $async.Future<$1.UpdateLocationResponse> update_Pre($grpc.ServiceCall $call, $async.Future<$1.UpdateLocationRequest> $request) async {
    return update($call, await $request);
  }

  $async.Future<$1.DeleteLocationResponse> delete_Pre($grpc.ServiceCall $call, $async.Future<$1.DeleteLocationRequest> $request) async {
    return delete($call, await $request);
  }

  $async.Future<$1.AddHallResponse> add_hall_Pre($grpc.ServiceCall $call, $async.Future<$1.AddHallRequest> $request) async {
    return add_hall($call, await $request);
  }

  $async.Future<$1.RemoveHallResponse> remove_hall_Pre($grpc.ServiceCall $call, $async.Future<$1.RemoveHallRequest> $request) async {
    return remove_hall($call, await $request);
  }

  $async.Future<$1.UpdateHallResponse> update_hall_Pre($grpc.ServiceCall $call, $async.Future<$1.UpdateHallRequest> $request) async {
    return update_hall($call, await $request);
  }

  $async.Future<$1.LocationView> get($grpc.ServiceCall call, $1.LocationRequest request);
  $async.Future<$1.LocationListView> list($grpc.ServiceCall call, $1.LocationListRequest request);
  $async.Future<$1.CreateLocationResponse> create($grpc.ServiceCall call, $1.CreateLocationRequest request);
  $async.Future<$1.UpdateLocationResponse> update($grpc.ServiceCall call, $1.UpdateLocationRequest request);
  $async.Future<$1.DeleteLocationResponse> delete($grpc.ServiceCall call, $1.DeleteLocationRequest request);
  $async.Future<$1.AddHallResponse> add_hall($grpc.ServiceCall call, $1.AddHallRequest request);
  $async.Future<$1.RemoveHallResponse> remove_hall($grpc.ServiceCall call, $1.RemoveHallRequest request);
  $async.Future<$1.UpdateHallResponse> update_hall($grpc.ServiceCall call, $1.UpdateHallRequest request);
}
