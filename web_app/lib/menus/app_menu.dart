import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ui_provider.dart' as ui;

class AppMenu extends StatelessWidget {
  const AppMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return MenuAnchor(
      builder: (context, controller, child) {
        return IconButton(
          onPressed: () {
            if (controller.isOpen) {
              controller.close();
            } else {
              controller.open();
            }
          },
          icon: const Icon(Icons.more_vert),
          tooltip: 'Меню',
        );
      },
      menuChildren: [
        // Theme section
        _buildThemeSection(context),
        const Divider(height: 8),
      ],
    );
  }

  Widget _buildThemeSection(BuildContext context) {
    final uiProvider = context.watch<ui.UIProvider>();

    return SubmenuButton(
      leadingIcon: Icon(_getThemeIcon(uiProvider.themeMode)),
      menuChildren: [
        MenuItemButton(
          leadingIcon: Icon(
            Icons.brightness_auto,
            color: uiProvider.themeMode == ui.ThemeMode.system
                ? Theme.of(context).colorScheme.primary
                : null,
          ),
          trailingIcon: uiProvider.themeMode == ui.ThemeMode.system
              ? const Icon(Icons.check, size: 18)
              : null,
          child: const Text('Системная'),
          onPressed: () {
            uiProvider.setThemeMode(ui.ThemeMode.system);
          },
        ),
        MenuItemButton(
          leadingIcon: Icon(
            Icons.light_mode,
            color: uiProvider.themeMode == ui.ThemeMode.light
                ? Theme.of(context).colorScheme.primary
                : null,
          ),
          trailingIcon: uiProvider.themeMode == ui.ThemeMode.light
              ? const Icon(Icons.check, size: 18)
              : null,
          child: const Text('Светлая'),
          onPressed: () {
            uiProvider.setThemeMode(ui.ThemeMode.light);
          },
        ),
        MenuItemButton(
          leadingIcon: Icon(
            Icons.dark_mode,
            color: uiProvider.themeMode == ui.ThemeMode.dark
                ? Theme.of(context).colorScheme.primary
                : null,
          ),
          trailingIcon: uiProvider.themeMode == ui.ThemeMode.dark
              ? const Icon(Icons.check, size: 18)
              : null,
          child: const Text('Тёмная'),
          onPressed: () {
            uiProvider.setThemeMode(ui.ThemeMode.dark);
          },
        ),
      ],
      child: const Text('Тема'),
    );
  }

  IconData _getThemeIcon(ui.ThemeMode mode) {
    switch (mode) {
      case ui.ThemeMode.system:
        return Icons.brightness_auto;
      case ui.ThemeMode.light:
        return Icons.light_mode;
      case ui.ThemeMode.dark:
        return Icons.dark_mode;
    }
  }
}
