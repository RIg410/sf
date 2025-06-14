import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:sf/desktop.dart';
import 'package:sf/mobile.dart';
import 'package:sf/providers/app_state.dart';
import 'package:sf/providers/auth_provider.dart';
import 'package:sf/providers/location_provider.dart';
import 'package:sf/providers/navigation_provider.dart';
import 'package:sf/providers/ui_provider.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        // Core app state
        ChangeNotifierProvider(create: (_) => AppState()),

        // UI state
        ChangeNotifierProvider(create: (_) => UIProvider()),

        // Navigation state
        ChangeNotifierProvider(create: (_) => NavigationProvider()),

        // Location provider with auto-load
        ChangeNotifierProvider(
          create: (_) => LocationProvider()..loadLocations(),
        ),

        // Auth provider depends on AppState
        ChangeNotifierProxyProvider<AppState, AuthProvider>(
          create: (context) => AuthProvider(appState: context.read<AppState>()),
          update: (context, appState, previous) =>
              previous ?? AuthProvider(appState: appState),
        ),
      ],
      child: Consumer<UIProvider>(
        builder: (context, uiProvider, child) {
          return MaterialApp(
            title: 'SoulFamily',
            theme: _buildTheme(uiProvider.isDarkMode),
            locale: uiProvider.locale,
            home: const AppWrapper(),
          );
        },
      ),
    );
  }

  ThemeData _buildTheme(bool isDarkMode) {
    if (isDarkMode) {
      return ThemeData(
        brightness: Brightness.dark,
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color.fromARGB(255, 255, 255, 255),
          brightness: Brightness.dark,
        ),
      );
    }
    return ThemeData(
      brightness: Brightness.light,
      colorScheme: ColorScheme.fromSeed(
        seedColor: const Color.fromARGB(255, 255, 255, 255),
        brightness: Brightness.light,
      ),
    );
  }
}

class AppWrapper extends StatefulWidget {
  const AppWrapper({super.key});

  @override
  State<AppWrapper> createState() => _AppWrapperState();
}

class _AppWrapperState extends State<AppWrapper> with WidgetsBindingObserver {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    
    // Initialize app state and system theme
    WidgetsBinding.instance.addPostFrameCallback((_) {
      context.read<AppState>().initialize();
      
      // Set initial system theme
      final brightness = WidgetsBinding.instance.window.platformBrightness;
      context.read<UIProvider>().updateSystemTheme(brightness == Brightness.dark);
    });
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangePlatformBrightness() {
    // Update system theme when platform brightness changes
    final brightness = WidgetsBinding.instance.window.platformBrightness;
    context.read<UIProvider>().updateSystemTheme(brightness == Brightness.dark);
  }

  @override
  Widget build(BuildContext context) {
    final appState = context.watch<AppState>();

    // Show loading screen while initializing
    if (!appState.isInitialized) {
      return const Scaffold(body: Center(child: CircularProgressIndicator()));
    }

    return const MainPage();
  }
}

class MainPage extends StatelessWidget {
  const MainPage({super.key});
  @override
  Widget build(BuildContext context) {
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      DeviceOrientation.portraitDown,
    ]);

    return LayoutBuilder(
      builder: (context, constraints) {
        if (constraints.maxWidth > 600) {
          return const DesktopPage();
        } else {
          return const MobilePage();
        }
      },
    );
  }
}
