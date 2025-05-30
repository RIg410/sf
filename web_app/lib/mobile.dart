import 'package:flutter/material.dart';
import 'package:web_app/common.dart';

class MobilePage extends StatelessWidget {
  const MobilePage({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SoulFamily')),
      drawer: Drawer(
        child: ListView(
          padding: EdgeInsets.zero,
          children: [
            const DrawerHeader(child: Text('Меню')),
            ListTile(title: const Text('Home'), onTap: () {}),
            ListTile(title: const Text('Classes'), onTap: () {}),
            ListTile(title: const Text('Instructors'), onTap: () {}),
            ListTile(title: const Text('Contact'), onTap: () {}),
          ],
        ),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: const [
            BannerSection(),
            SizedBox(height: 24),
            NewsOffersSection(),
            SizedBox(height: 24),
            ScheduleSection(),
            SizedBox(height: 24),
            InstructorsSection(),
            SizedBox(height: 24),
            ProgramsSection(),
          ],
        ),
      ),
    );
  }
}
