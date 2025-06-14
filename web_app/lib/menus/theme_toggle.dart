import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sf/providers/ui_provider.dart';
import '../providers/ui_provider.dart' as ui;

class ThemeToggle extends StatelessWidget {
  const ThemeToggle({super.key});

  @override
  Widget build(BuildContext context) {
    final uiProvider = context.watch<ui.UIProvider>();

    IconData icon;
    String tooltip;

    switch (uiProvider.themeMode) {
      case ui.ThemeMode.system:
        icon = Icons.brightness_auto;
        tooltip = 'Автоматическая тема';
        break;
      case ui.ThemeMode.light:
        icon = Icons.light_mode;
        tooltip = 'Светлая тема';
        break;
      case ui.ThemeMode.dark:
        icon = Icons.dark_mode;
        tooltip = 'Тёмная тема';
        break;
    }

    return IconButton(
      icon: Icon(icon),
      tooltip: tooltip,
      onPressed: () {
        _showThemeMenu(context, uiProvider);
      },
    );
  }

  void _showThemeMenu(BuildContext context, ui.UIProvider uiProvider) {
    showMenu<ui.ThemeMode>(
      context: context,
      position: RelativeRect.fromLTRB(
        MediaQuery.of(context).size.width - 200,
        kToolbarHeight,
        0,
        0,
      ),
      items: [
        PopupMenuItem(
          value: ui.ThemeMode.system,
          child: Row(
            children: [
              Icon(
                Icons.brightness_auto,
                color: uiProvider.themeMode == ui.ThemeMode.system
                    ? Theme.of(context).colorScheme.primary
                    : null,
              ),
              const SizedBox(width: 12),
              const Text('Системная'),
            ],
          ),
        ),
        PopupMenuItem(
          value: ui.ThemeMode.light,
          child: Row(
            children: [
              Icon(
                Icons.light_mode,
                color: uiProvider.themeMode == ui.ThemeMode.light
                    ? Theme.of(context).colorScheme.primary
                    : null,
              ),
              const SizedBox(width: 12),
              const Text('Светлая'),
            ],
          ),
        ),
        PopupMenuItem(
          value: ui.ThemeMode.dark,
          child: Row(
            children: [
              Icon(
                Icons.dark_mode,
                color: uiProvider.themeMode == ui.ThemeMode.dark
                    ? Theme.of(context).colorScheme.primary
                    : null,
              ),
              const SizedBox(width: 12),
              const Text('Тёмная'),
            ],
          ),
        ),
      ],
    ).then((value) {
      if (value != null) {
        uiProvider.setThemeMode(value);
      }
    });
  }
}

class ThemeToggleSwitch extends StatelessWidget {
  const ThemeToggleSwitch({super.key});

  @override
  Widget build(BuildContext context) {
    final uiProvider = context.watch<UIProvider>();

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(
          Icons.light_mode,
          size: 20,
          color: uiProvider.isDarkMode
              ? Theme.of(context).colorScheme.onSurfaceVariant.withOpacity(0.5)
              : Theme.of(context).colorScheme.primary,
        ),
        const SizedBox(width: 8),
        Switch(
          value: uiProvider.isDarkMode,
          onChanged: (value) {
            uiProvider.setDarkMode(value);
          },
        ),
        const SizedBox(width: 8),
        Icon(
          Icons.dark_mode,
          size: 20,
          color: uiProvider.isDarkMode
              ? Theme.of(context).colorScheme.primary
              : Theme.of(context).colorScheme.onSurfaceVariant.withOpacity(0.5),
        ),
      ],
    );
  }
}
