import 'package:flutter/material.dart';

enum ThemeMode {
  system,
  light,
  dark,
}

class UIProvider extends ChangeNotifier {
  bool _isSidebarOpen = false;
  ThemeMode _themeMode = ThemeMode.system;
  bool _systemIsDark = false;
  double _textScaleFactor = 1.0;
  Locale _locale = const Locale('ru');

  // Getters
  bool get isSidebarOpen => _isSidebarOpen;
  ThemeMode get themeMode => _themeMode;
  bool get isDarkMode {
    switch (_themeMode) {
      case ThemeMode.system:
        return _systemIsDark;
      case ThemeMode.light:
        return false;
      case ThemeMode.dark:
        return true;
    }
  }
  double get textScaleFactor => _textScaleFactor;
  Locale get locale => _locale;

  // Sidebar management
  void toggleSidebar() {
    _isSidebarOpen = !_isSidebarOpen;
    notifyListeners();
  }

  void openSidebar() {
    if (!_isSidebarOpen) {
      _isSidebarOpen = true;
      notifyListeners();
    }
  }

  void closeSidebar() {
    if (_isSidebarOpen) {
      _isSidebarOpen = false;
      notifyListeners();
    }
  }

  // System theme detection
  void updateSystemTheme(bool isDark) {
    if (_systemIsDark != isDark) {
      _systemIsDark = isDark;
      if (_themeMode == ThemeMode.system) {
        notifyListeners();
      }
    }
  }

  // Theme management
  void toggleTheme() {
    switch (_themeMode) {
      case ThemeMode.system:
        _themeMode = _systemIsDark ? ThemeMode.light : ThemeMode.dark;
        break;
      case ThemeMode.light:
        _themeMode = ThemeMode.dark;
        break;
      case ThemeMode.dark:
        _themeMode = ThemeMode.light;
        break;
    }
    notifyListeners();
  }

  void setThemeMode(ThemeMode mode) {
    if (_themeMode != mode) {
      _themeMode = mode;
      notifyListeners();
    }
  }

  void setDarkMode(bool value) {
    _themeMode = value ? ThemeMode.dark : ThemeMode.light;
    notifyListeners();
  }

  // Text scale management
  void setTextScaleFactor(double factor) {
    if (factor >= 0.8 && factor <= 1.4 && _textScaleFactor != factor) {
      _textScaleFactor = factor;
      notifyListeners();
    }
  }

  void increaseTextScale() {
    if (_textScaleFactor < 1.4) {
      _textScaleFactor = (_textScaleFactor + 0.1).clamp(0.8, 1.4);
      notifyListeners();
    }
  }

  void decreaseTextScale() {
    if (_textScaleFactor > 0.8) {
      _textScaleFactor = (_textScaleFactor - 0.1).clamp(0.8, 1.4);
      notifyListeners();
    }
  }

  void resetTextScale() {
    if (_textScaleFactor != 1.0) {
      _textScaleFactor = 1.0;
      notifyListeners();
    }
  }

  // Locale management
  void setLocale(Locale locale) {
    if (_locale != locale) {
      _locale = locale;
      notifyListeners();
    }
  }
}