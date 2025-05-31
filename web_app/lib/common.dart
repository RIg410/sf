import 'package:flutter/material.dart';

class BannerSection extends StatelessWidget {
  const BannerSection();
  @override
  Widget build(BuildContext context) {
    return Card(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // В реале можно подгружать любые изображения
          ClipRRect(
            borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
            child: Image.asset(
              'fox1.png',
              fit: BoxFit.cover,
              height: 600,
              width: double.infinity,
            ),
          ),
          const Padding(
            padding: EdgeInsets.all(16),
            child: Text(
              'Добро пожаловать в Soul Family',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.w600),
            ),
          ),
          const Padding(
            padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: ElevatedButton(
              onPressed: null,
              child: Text('Узнать больше'),
            ),
          ),
        ],
      ),
    );
  }
}

class NewsOffersSection extends StatelessWidget {
  const NewsOffersSection();
  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Новости и предложения',
          style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children: [
            _SimpleCard(
              color: Colors.teal.shade300,
              title: 'Новый курс',
              subtitle: 'Начало 1 августа',
            ),
            _SimpleCard(
              color: Colors.orange.shade300,
              title: 'Скидка 30%',
              subtitle: 'На первый месяц',
            ),
          ],
        ),
      ],
    );
  }
}

class ScheduleSection extends StatelessWidget {
  const ScheduleSection();
  @override
  Widget build(BuildContext context) {
    // Пример простого расписания
    final days = ['Пн', 'Вт', 'Чт', 'Ср', 'Пт'];
    final times = ['5:00 AM', '10:50 AM', '7:00 PM', '8:00 AM', '6:30 PM'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Расписание',
          style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 12),
        Table(
          border: TableBorder.all(color: Colors.grey.shade300),
          children: [
            TableRow(
              children: days
                  .map(
                    (d) => Padding(
                      padding: const EdgeInsets.all(8),
                      child: Text(
                        d,
                        style: const TextStyle(fontWeight: FontWeight.bold),
                      ),
                    ),
                  )
                  .toList(),
            ),
            TableRow(
              children: times
                  .map(
                    (t) => Padding(
                      padding: const EdgeInsets.all(8),
                      child: Text(t),
                    ),
                  )
                  .toList(),
            ),
          ],
        ),
      ],
    );
  }
}

class InstructorsSection extends StatelessWidget {
  const InstructorsSection();
  @override
  Widget build(BuildContext context) {
    final instructors = [
      {'name': 'Emma', 'img': 'https://i.pravatar.cc/150?img=12'},
      {'name': 'Sarah', 'img': 'https://i.pravatar.cc/150?img=32'},
      {'name': 'Alex', 'img': 'https://i.pravatar.cc/150?img=48'},
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Наши инструкторы',
          style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: instructors.map((ins) {
            return Column(
              children: [
                CircleAvatar(
                  radius: 40,
                  backgroundImage: NetworkImage(ins['img']!),
                ),
                const SizedBox(height: 8),
                Text(
                  ins['name']!,
                  style: const TextStyle(fontWeight: FontWeight.w500),
                ),
              ],
            );
          }).toList(),
        ),
      ],
    );
  }
}

class ProgramsSection extends StatelessWidget {
  const ProgramsSection();
  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Программы тренировок',
          style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children: [
            _IconCard(
              color: Colors.purple.shade300,
              title: 'Йога для начинающих',
              icon: Icons.self_improvement,
            ),
            _IconCard(
              color: Colors.lightBlue.shade300,
              title: 'Йога для продвинутых',
              icon: Icons.fitness_center,
            ),
          ],
        ),
      ],
    );
  }
}

// Простейшая карточка
class _SimpleCard extends StatelessWidget {
  final Color color;
  final String title, subtitle;
  const _SimpleCard({
    required this.color,
    required this.title,
    required this.subtitle,
  });
  @override
  Widget build(BuildContext context) {
    return Container(
      width: 140,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        children: [
          Text(
            title,
            style: const TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          Text(subtitle, style: const TextStyle(color: Colors.white70)),
        ],
      ),
    );
  }
}

// Карточка с иконкой
class _IconCard extends StatelessWidget {
  final Color color;
  final String title;
  final IconData icon;
  const _IconCard({
    required this.color,
    required this.title,
    required this.icon,
  });
  @override
  Widget build(BuildContext context) {
    return Container(
      width: 140,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        children: [
          Icon(icon, size: 48, color: Colors.white),
          const SizedBox(height: 12),
          Text(
            title,
            style: const TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }
}
