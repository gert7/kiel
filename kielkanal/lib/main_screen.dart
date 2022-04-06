import 'package:flutter/material.dart';
import 'package:kielkanal/config_screen/config_screen.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({Key? key}) : super(key: key);

  static const List _options = <Widget>[
    ConfigScreen(),
    ConfigScreen(),
    ConfigScreen()
  ];

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int _selectedItem = 0;

  void selectItem(int i) {
    setState(() {
      _selectedItem = i;
    });
  }

  @override
  Widget build(BuildContext context) {
    final yellow = Colors.amber[200] ?? Colors.yellow;

    // TODO: implement build
    return SafeArea(
      child: Scaffold(
        body: MainScreen._options[_selectedItem],
        bottomNavigationBar: BottomNavigationBar(
          items: const <BottomNavigationBarItem>[
            BottomNavigationBarItem(
                icon: Icon(Icons.calendar_today), label: "Nädalapäevad"),
            BottomNavigationBarItem(
                icon: Icon(Icons.access_time_rounded), label: "Täna ja homme"),
            BottomNavigationBarItem(
                icon: Icon(Icons.notifications), label: "Teated"),
          ],
          onTap: selectItem,
        ),
      ),
    );
  }
}