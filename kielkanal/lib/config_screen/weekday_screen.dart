import 'package:flutter/material.dart';
import 'package:kielkanal/config_screen/kiel_form.dart';
import 'package:kielkanal/config_screen/weekly_overrides.dart';

class WeekdayScreen extends StatefulWidget {
  final int dayNumber;

  const WeekdayScreen(this.dayNumber, {Key? key}) : super(key: key);

  @override
  State<WeekdayScreen> createState() => _WeekdayScreenState();
}

class _WeekdayScreenState extends State<WeekdayScreen>
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
    // final sample = getSample();
    // final back = TomlDocument.fromMap(sample.toMap());

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
