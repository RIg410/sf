import 'package:flutter/material.dart';
import '../generated/user.pb.dart';

enum AppStatus { initial, loading, authenticated, unauthenticated, error }

class AppState extends ChangeNotifier {
  AppStatus _status = AppStatus.initial;
  UserView? _currentUser;
  String? _authToken;
  String? _error;
  bool _isInitialized = false;

  // Getters
  AppStatus get status => _status;
  UserView? get currentUser => _currentUser;
  String? get authToken => _authToken;
  String? get error => _error;
  bool get isInitialized => _isInitialized;
  bool get isAuthenticated =>
      _status == AppStatus.authenticated && _currentUser != null;

  // Initialize app
  Future<void> initialize() async {
    if (_isInitialized) return;

    try {
      _status = AppStatus.loading;
      notifyListeners();

      _isInitialized = true;
      _status = AppStatus.unauthenticated;
      notifyListeners();
    } catch (e) {
      _error = e.toString();
      _status = AppStatus.error;
      notifyListeners();
    }
  }

  // Set authenticated user
  void setUser(UserView user, String token) {
    _currentUser = user;
    _authToken = token;
    _status = AppStatus.authenticated;
    _error = null;
    notifyListeners();
  }

  // Logout
  void logout() {
    _currentUser = null;
    _authToken = null;
    _status = AppStatus.unauthenticated;
    _error = null;
    notifyListeners();
  }

  // Set error
  void setError(String error) {
    _error = error;
    _status = AppStatus.error;
    notifyListeners();
  }

  // Clear error
  void clearError() {
    _error = null;
    if (_currentUser != null) {
      _status = AppStatus.authenticated;
    } else {
      _status = AppStatus.unauthenticated;
    }
    notifyListeners();
  }
}
