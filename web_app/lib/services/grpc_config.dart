import 'package:grpc/grpc_web.dart';
import 'constants.dart';

class GrpcConfig {
  static GrpcConfig? _instance;
  late GrpcWebClientChannel _channel;
  
  static String get url => AppConstants.grpcUrl;

  GrpcConfig._internal() {
    _channel = GrpcWebClientChannel.xhr(Uri.parse(AppConstants.grpcUrl));
  }

  static GrpcConfig get instance {
    _instance ??= GrpcConfig._internal();
    return _instance!;
  }

  GrpcWebClientChannel get channel => _channel;

  void dispose() {
    _channel.shutdown();
  }
}
