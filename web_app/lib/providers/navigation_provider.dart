import 'package:flutter/material.dart';

enum AppRoute {
  home,
  schedule,
  instructors,
  programs,
  prices,
  profile,
  login,
}

class NavigationProvider extends ChangeNotifier {
  AppRoute _currentRoute = AppRoute.home;
  final List<AppRoute> _history = [AppRoute.home];

  AppRoute get currentRoute => _currentRoute;
  List<AppRoute> get history => List.unmodifiable(_history);
  bool get canGoBack => _history.length > 1;

  void navigateTo(AppRoute route) {
    if (_currentRoute != route) {
      _currentRoute = route;
      _history.add(route);
      notifyListeners();
    }
  }

  void goBack() {
    if (canGoBack) {
      _history.removeLast();
      _currentRoute = _history.last;
      notifyListeners();
    }
  }

  void resetToHome() {
    _currentRoute = AppRoute.home;
    _history.clear();
    _history.add(AppRoute.home);
    notifyListeners();
  }

  String getRouteTitle(AppRoute route) {
    switch (route) {
      case AppRoute.home:
        return 'Главная';
      case AppRoute.schedule:
        return 'Расписание';
      case AppRoute.instructors:
        return 'Инструкторы';
      case AppRoute.programs:
        return 'Программы';
      case AppRoute.prices:
        return 'Цены';
      case AppRoute.profile:
        return 'Профиль';
      case AppRoute.login:
        return 'Вход';
    }
  }
}