import 'dart:async';

import 'package:flutter/material.dart';
import 'package:kielkanal/config_screen/config_screen.dart';
import 'package:kielkanal/database/config_file_db.dart';
import 'package:provider/provider.dart';

import 'config_controller/config_controller.dart';
import 'config_controller/config_file.dart';

class MainScreenDBFilter extends StatelessWidget {
  final String ip;
  final _streamController = StreamController<ConfigFile?>();

  void loadFromDatabase() async {
    _streamController.add(null);
    final result = await fetchFromDatabase(ip);
    _streamController.add(result);
  }

  MainScreenDBFilter(this.ip, {Key? key}) : super(key: key) {
    loadFromDatabase();
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<ConfigFile?>(
        stream: _streamController.stream,
        initialData: null,
        builder: (BuildContext context, snapshot) {
          final data = snapshot.data;
          if (data != null) {
            return MainScreen(ip, data);
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        });
  }
}

class MainScreen extends StatefulWidget {
  final String ip;
  final ConfigFile configFile;

  const MainScreen(this.ip, this.configFile, {Key? key}) : super(key: key);

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
    return ChangeNotifierProvider<ConfigController>(
      create: (BuildContext context) =>
          ConfigController.fromConfigFile(widget.configFile),
      child: SafeArea(
        child: Scaffold(
          body: MainScreen._options[_selectedItem],
          bottomNavigationBar: BottomNavigationBar(
            items: const <BottomNavigationBarItem>[
              BottomNavigationBarItem(
                  icon: Icon(Icons.calendar_today), label: "Nädalapäevad"),
              BottomNavigationBarItem(
                  icon: Icon(Icons.access_time_rounded),
                  label: "Täna ja homme"),
              BottomNavigationBarItem(
                  icon: Icon(Icons.notifications), label: "Teated"),
            ],
            onTap: selectItem,
          ),
        ),
      ),
    );
  }
}
