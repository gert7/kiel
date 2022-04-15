import 'dart:async';

import 'package:flutter/material.dart';
import 'package:kielkanal/config_screen/config_screen.dart';
import 'package:kielkanal/database/config_file_db.dart';
import 'package:provider/provider.dart';

import 'config_controller/config_controller.dart';
import 'config_controller/config_file.dart';
import 'day_screen/day_screen.dart';

class ReloadNotification extends Notification {
  final String message;
  ReloadNotification(this.message);
}

class MainScreenDBFilter extends StatefulWidget {
  final String ip;
  const MainScreenDBFilter(this.ip, {Key? key}) : super(key: key);

  @override
  State<MainScreenDBFilter> createState() => _MainScreenDBFilterState();
}

class _MainScreenDBFilterState extends State<MainScreenDBFilter> {
  final _streamController = StreamController<ConfigFile?>();
  String? loadingString;

  void loadFromDatabase() async {
    _streamController.add(null);
    print("Fetching from database");
    final result = await fetchConfigFileFromDatabase(widget.ip);
    _streamController.add(result);
  }

  @override
  void initState() {
    super.initState();
    loadFromDatabase();
  }

  @override
  Widget build(BuildContext context) {
    return NotificationListener<ReloadNotification>(
      onNotification: (notification) {
        _streamController.add(null);
        loadingString = notification.message;
        return true;
      },
      child: StreamBuilder<ConfigFile?>(
          stream: _streamController.stream,
          initialData: null,
          builder: (BuildContext context, snapshot) {
            final data = snapshot.data;
            print("Rebuilding MainScreenDBFilter...");
            if (data != null) {
              return MainScreen(widget.ip, data);
            } else {
              return Scaffold(
                body: Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    const CircularProgressIndicator(),
                    Padding(
                      padding: const EdgeInsets.only(top: 8.0),
                      child: Text(loadingString ?? "Laadimine", style: Theme.of(context).textTheme.subtitle2,),
                    )
                  ],
                )),
              );
            }
          }),
    );
  }
}

class MainScreen extends StatefulWidget {
  final String ip;
  final ConfigFile configFile;

  const MainScreen(this.ip, this.configFile, {Key? key}) : super(key: key);

  List get options => <Widget>[
    const ConfigScreen(),
    DayScreen(ip),
    const ConfigScreen()
  ];

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int _selectedItem = 0;

  void selectItem(int i) {
    print("selected $i");
    setState(() => _selectedItem = i);
  }

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider<ConfigController>(
      create: (BuildContext context) =>
          ConfigController.fromConfigFile(widget.configFile),
      child: SafeArea(
        child: Scaffold(
          body: widget.options[_selectedItem],
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
            currentIndex: _selectedItem,
            onTap: selectItem,
          ),
        ),
      ),
    );
  }
}
