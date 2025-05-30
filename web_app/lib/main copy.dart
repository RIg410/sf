import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => MyAppState(),
      child: MaterialApp(
        title: 'Soul Family',
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(
            seedColor: const Color.fromARGB(255, 249, 0, 0),
          ),
        ),
        home: MyHomePage(),
      ),
    );
  }
}

class MyAppState extends ChangeNotifier {
  int counter = 0;
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    int selectedIndex = 0;

    var size = MediaQuery.of(context).size;

    NavigationBar bottomNavigationBar = NavigationBar(
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
        setState(() {
          selectedIndex = value;
          print('Selected index: $selectedIndex');
        });
      },
    );

    // LayoutBuilder(
    //   builder: (context, constraints) {
    //     setState(() {
    //       size = constraints.biggest;
    //     });

    //     if (size.width > 600) {
    //       // Большая страница
    //       return BigScreenNavigation();
    //     } else {
    //       // Маленькая страница
    //       return SmallScreenNavigation();
    //     }
    //   },
    // );

    return Scaffold(
      appBar: AppBar(title: const Text('Soul Family')),
      body: Column(children: []),
      bottomNavigationBar: bottomNavigationBar,
    );
  }
}
