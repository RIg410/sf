import 'package:flutter/material.dart';
import 'package:sf/common.dart';
import 'package:sf/logo.dart';

class DesktopPage extends StatelessWidget {
  const DesktopPage({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Logo(),
        centerTitle: false,
        actions: [
          TextButton(onPressed: () {}, child: const Text('Домой')),
          TextButton(onPressed: () {}, child: const Text('Расписание')),
          TextButton(onPressed: () {}, child: const Text('Инструкторы')),
          TextButton(onPressed: () {}, child: const Text('Программы')),
          TextButton(onPressed: () {}, child: const Text('Цены')),
          const SizedBox(width: 16),
          IconButton(onPressed: () {}, icon: const Icon(Icons.account_circle)),
          const SizedBox(width: 24),
        ],
      ),
      body: const Row(
        children: [
          Expanded(
            flex: 2,
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: const [
                  BannerSection(),
                  SizedBox(height: 32),
                  NewsOffersSection(),
                  SizedBox(height: 32),
                  InstructorsSection(),
                ],
              ),
            ),
          ),
          Expanded(
            flex: 1,
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24),
              child: Column(
                children: const [
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
