import 'package:flutter/material.dart';
import 'package:sf/common.dart';
import 'package:sf/home/locations.dart';

class MobilePage extends StatelessWidget {
  const MobilePage({super.key});

  @override
  Widget build(BuildContext context) {
    int selectedIndex = 0;

    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(40),
        child: AppBar(
          title: const LocationsSection(showAddress: false, isMobile: true),
        ),
      ),
      bottomNavigationBar: NavigationBar(
        destinations: const [
          NavigationDestination(icon: Icon(Icons.home), label: 'Дом'),
          NavigationDestination(icon: Icon(Icons.person), label: 'Профиль'),
          NavigationDestination(
            icon: Icon(Icons.calendar_month),
            label: 'Расписание',
          ),
          NavigationDestination(icon: Icon(Icons.more_horiz), label: 'Еще'),
        ],
        selectedIndex: selectedIndex,
        onDestinationSelected: (value) {
          selectedIndex = value;
        },
      ),
      body: const SingleChildScrollView(
        padding: EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            BannerSection(),
            SizedBox(height: 24),
            NewsOffersSection(),
            SizedBox(height: 24),
            ScheduleSection(),
            SizedBox(height: 24),
            InstructorsSection(),
            SizedBox(height: 24),
            ProgramsSection(),
            SizedBox(height: 24),
            // LocationsSection moved to AppBar,
          ],
        ),
      ),
    );
  }
}
