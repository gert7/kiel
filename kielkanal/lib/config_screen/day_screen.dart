import 'package:flutter/material.dart';
import 'package:kielkanal/config_controller/config_controller.dart';
import 'package:kielkanal/config_controller/config_file.dart';
import 'package:kielkanal/config_screen/kiel_form.dart';
import 'package:kielkanal/config_screen/weekly_overrides.dart';
import 'package:provider/provider.dart';
import 'package:toml/toml.dart';

class DayScreen extends StatefulWidget {
  final int dayNumber;

  const DayScreen(this.dayNumber, {Key? key}) : super(key: key);

  @override
  State<DayScreen> createState() => _DayScreenState();
}

class _DayScreenState extends State<DayScreen>
    with SingleTickerProviderStateMixin {
  static const tabs = <Tab>[
    Tab(
      text: "Alus",
    ),
    Tab(
      text: "Strateegia",
    ),
    Tab(text: "Käsitsi"),
  ];

  late TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: tabs.length, vsync: this);
  }

  @override
  Widget build(BuildContext context) {
    final sample = getSample();
    final back = TomlDocument.fromMap(sample.toMap());

    return Column(
      children: [
        TabBar(
          tabs: tabs,
          controller: _tabController,
          labelColor: Colors.black,
        ),
        Expanded(
          child: TabBarView(controller: _tabController, children: [
            KielForm(widget.dayNumber, KielFormType.base),
            KielForm(widget.dayNumber, KielFormType.strategy),
            WeeklyOverrideWidget(widget.dayNumber),
          ]),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }
}
