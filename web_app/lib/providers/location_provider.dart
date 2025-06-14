import 'package:flutter/material.dart';
import '../generated/locations.pb.dart';
import '../services/locations.dart';

class LocationProvider extends ChangeNotifier {
  final LocationsService _locationsService = LocationsService();

  List<LocationView>? _locations;
  LocationView? _selectedLocation;
  bool _isLoading = false;
  String? _error;

  List<LocationView>? get locations => _locations;
  LocationView? get selectedLocation => _selectedLocation;
  bool get isLoading => _isLoading;
  String? get error => _error;

  Future<void> loadLocations() async {
    if (_locations != null) return;

    try {
      _isLoading = true;
      _error = null;
      notifyListeners();

      final response = await _locationsService.getLocations();
      _locations = response.locations;

      if (_locations != null && _locations!.isNotEmpty) {
        _selectedLocation = _locations!.first;
      }

      _isLoading = false;
      notifyListeners();
    } catch (e) {
      _error = e.toString();
      _isLoading = false;
      notifyListeners();
    }
  }

  void selectLocation(LocationView location) {
    if (_selectedLocation?.id != location.id) {
      _selectedLocation = location;
      notifyListeners();
    }
  }
}
