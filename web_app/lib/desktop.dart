import 'package:flutter/material.dart';
import 'package:web_app/common.dart';

class DesktopPage extends StatelessWidget {
  const DesktopPage({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SoulFamily'),
        centerTitle: false,
        actions: [
          TextButton(
            onPressed: () {},
            child: const Text('Home', style: TextStyle(color: Colors.white)),
          ),
          TextButton(
            onPressed: () {},
            child: const Text('Classes', style: TextStyle(color: Colors.white)),
          ),
          TextButton(
            onPressed: () {},
            child: const Text(
              'Instructors',
              style: TextStyle(color: Colors.white),
            ),
          ),
          TextButton(
            onPressed: () {},
            child: const Text('Contact', style: TextStyle(color: Colors.white)),
          ),
          const SizedBox(width: 16),
          IconButton(onPressed: () {}, icon: const Icon(Icons.account_circle)),
          IconButton(onPressed: () {}, icon: const Icon(Icons.favorite_border)),
          const SizedBox(width: 24),
        ],
      ),
      body: Row(
        children: [
          // Левый большой Banner
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
          // Правый sidebar с расписанием и программами
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
