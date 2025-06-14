import '../generated/locations.pbgrpc.dart';
import 'grpc_config.dart';

class LocationsService {
  late LocationsServiceClient _client;

  LocationsService() {
    _client = LocationsServiceClient(GrpcConfig.instance.channel);
  }

  Future<LocationListView> getLocations() async {
    try {
      print('Fetching locations...');
      final request = LocationListRequest();
      final response = await _client.list(request);
      print('Received ${response.locations.length} locations');
      return response;
    } catch (e) {
      throw Exception('Failed to load locations: $e');
    }
  }
}
