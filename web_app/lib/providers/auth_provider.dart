import 'package:flutter/material.dart';
import 'package:grpc/grpc_web.dart';
import '../generated/auth.pbgrpc.dart';
import '../generated/users.pbgrpc.dart';
import '../generated/user.pb.dart';
import '../services/grpc_config.dart';
import 'app_state.dart';

class AuthProvider extends ChangeNotifier {
  final AppState appState;

  AuthProvider({required this.appState});

  bool _isLoading = false;
  String? _error;

  bool get isLoading => _isLoading;
  String? get error => _error;

  // Login with Telegram
  Future<bool> loginWithTelegram(String telegramData) async {
    try {
      _isLoading = true;
      _error = null;
      notifyListeners();

      final channel = GrpcWebClientChannel.xhr(Uri.parse(GrpcConfig.url));
      final authClient = AuthServiceClient(channel);

      // final request = LoginRequest()..telegramData = telegramData;
      // final response = await authClient.login(request);

      // if (response.hasToken() && response.hasUser()) {
      //   appState.setUser(response.user, response.token);
      //   await channel.shutdown();
      //   _isLoading = false;
      //   notifyListeners();
      //   return true;
      // }

      await channel.shutdown();
      _error = 'Invalid response from server';
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = e.toString();
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  // Get current user info
  Future<void> getCurrentUser() async {
    if (appState.authToken == null) return;

    try {
      _isLoading = true;
      _error = null;
      notifyListeners();

      final channel = GrpcWebClientChannel.xhr(Uri.parse(GrpcConfig.url));
      final usersClient = UsersServiceClient(
        channel,
        options: CallOptions(
          metadata: {'authorization': 'Bearer ${appState.authToken}'},
        ),
      );

      // final request = GetMeRequest();
      // final response = await usersClient.getMe(request);

      // if (response.hasUser()) {
      //   appState.setUser(response.user, appState.authToken!);
      // }

      await channel.shutdown();
      _isLoading = false;
      notifyListeners();
    } catch (e) {
      _error = e.toString();
      _isLoading = false;
      if (e.toString().contains('UNAUTHENTICATED')) {
        appState.logout();
      }
      notifyListeners();
    }
  }

  // Logout
  void logout() {
    appState.logout();
    notifyListeners();
  }

  // Clear error
  void clearError() {
    _error = null;
    notifyListeners();
  }
}
