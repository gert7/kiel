import 'package:flutter/material.dart';

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
    return Column(
      children: [
        TabBar(
          tabs: tabs,
          controller: _tabController,
          labelColor: Colors.black,
        ),
        Expanded(
          child: TabBarView(controller: _tabController, children: [
            Text("hi"),
            Text("Hello"),
            Text("What up"),
          ]),
        )
      ],
    );
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }
}
