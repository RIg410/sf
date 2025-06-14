import 'package:flutter/material.dart';
import 'package:sf/common.dart';
import 'package:sf/home/logo.dart';
import 'package:sf/menus/app_menu.dart';

class DesktopPage extends StatelessWidget {
  const DesktopPage({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Logo(),
        centerTitle: false,
        actions: [
          TextButton(onPressed: () {}, child: const Text('Расписание')),
          TextButton(onPressed: () {}, child: const Text('Инструкторы')),
          TextButton(onPressed: () {}, child: const Text('Программы')),
          TextButton(onPressed: () {}, child: const Text('Цены')),
          const SizedBox(width: 16),
          IconButton(onPressed: () {}, icon: const Icon(Icons.account_circle)),
          const SizedBox(width: 8),
          const AppMenu(),
          const SizedBox(width: 16),
        ],
      ),
      body: const Row(
        children: [
          Expanded(
            flex: 2,
            child: SingleChildScrollView(
              padding: EdgeInsets.all(24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  BannerSection(),
                  SizedBox(height: 32),
                  NewsOffersSection(),
                  SizedBox(height: 32),
                  InstructorsSection(),
                  SizedBox(height: 32),
                  // LocationsSection moved to AppBar,
                ],
              ),
            ),
          ),
          Expanded(
            flex: 1,
            child: SingleChildScrollView(
              padding: EdgeInsets.all(24),
              child: Column(
                children: [
                  ScheduleSection(),
                  SizedBox(height: 32),
                  ProgramsSection(),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
