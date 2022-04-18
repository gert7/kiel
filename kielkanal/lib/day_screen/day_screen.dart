import 'dart:async';

import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:kielkanal/database/day_summary_db.dart';
import 'package:kielkanal/day_screen/day_table.dart';
import 'package:timezone/timezone.dart';

import '../database/constants.dart';

String getMonthName(int month) {
  const months = [
    "Jaanuar",
    "Veebruar",
    "Märts",
    "Aprill",
    "Mai",
    "Juuni",
    "Juuli",
    "August",
    "September",
    "Oktoober",
    "November",
    "Detsember",
  ];
  return months[month - 1];
}

class DayScreenDraft extends StatefulWidget {
  final String ip;

  static TZDateTime marketDayFromNow(int dayOffset) {
    final marketNow =
    TZDateTime.now(marketTimeZone()).add(Duration(days: dayOffset));
    return TZDateTime(
        marketTimeZone(), marketNow.year, marketNow.month, marketNow.day);
  }

  const DayScreenDraft(this.ip, {Key? key}) : super(key: key);

  @override
  State<DayScreenDraft> createState() => _DayScreenDraftState();
}

class _DayScreenDraftState extends State<DayScreenDraft>
    with SingleTickerProviderStateMixin {
  final _daysStreamController =
  StreamController<TodayTomorrowSummary>.broadcast();

  late TZDateTime today;
  late TZDateTime tomorrow;

  static const tabs = <Tab>[
    Tab(
      text: "Täna",
    ),
    Tab(
      text: "Homme",
    )
  ];

  late TabController _tabController;

  void loadData(String ip) async {
    final states0 = await PowerStateDB.getDay(ip, today);
    final prices0 = await PriceCellDB.getDay(ip, today);
    final states1 = await PowerStateDB.getDay(ip, tomorrow);
    final prices1 = await PriceCellDB.getDay(ip, tomorrow);
    _daysStreamController
        .add(TodayTomorrowSummary(states0, prices0, states1, prices1));
  }

  @override
  void initState() {
    super.initState();
    today = DayScreenDraft.marketDayFromNow(0);
    tomorrow = DayScreenDraft.marketDayFromNow(1);
    _tabController = TabController(length: tabs.length, vsync: this);
    loadData(widget.ip);
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<TodayTomorrowSummary>(builder: (context, snapshot) {
      if (snapshot.hasData) {
        final data = snapshot.data!;
        return Column(
          children: [
            TabBar(
              tabs: tabs,
              controller: _tabController,
              labelColor: Colors.black,
            ),
            Expanded(
              child: TabBarView(controller: _tabController, children: [
                DayScreenTable(data.today(), today),
                DayScreenTable(data.tomorrow(), tomorrow),
              ]),
            ),
          ],
        );
      } else {
        return const Center(child: CircularProgressIndicator());
      }
    },
    stream: _daysStreamController.stream,);
  }
}
