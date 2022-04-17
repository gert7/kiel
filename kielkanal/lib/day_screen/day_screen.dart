import 'dart:async';

import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:kielkanal/database/day_summary_db.dart';
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

class DayScreenFront extends StatefulWidget {
  final String ip;
  const DayScreenFront(this.ip, {Key? key}) : super(key: key);

  @override
  State<DayScreenFront> createState() => _DayScreenFrontState();
}

class _DayScreenFrontState extends State<DayScreenFront> with SingleTickerProviderStateMixin {
  static const tabs = <Tab>[
    Tab(
      text: "Täna",
    ),
    Tab(
      text: "Homme",
    )
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
            DayScreen(widget.ip, 0),
            DayScreen(widget.ip, 1),
          ]),
        ),
      ],
    );
  }
}

class DayScreen extends StatelessWidget {
  final _daySummaryStream = StreamController<DaySummary>();
  final int daysOffset;

  static const rowStyleBold =
      TextStyle(fontSize: 16, fontWeight: FontWeight.bold);
  static const rowStyle = TextStyle(fontSize: 16);

  static TZDateTime marketToday() {
    final marketNow = TZDateTime.now(marketTimeZone());
    return TZDateTime(
        marketTimeZone(), marketNow.year, marketNow.month, marketNow.day);
  }

  void loadData(String ip) async {
    final today = marketToday();
    final states = await PowerStateDB.getDay(ip, today);
    final prices = await PriceCellDB.getDay(ip, today);
    _daySummaryStream.add(DaySummary(states, prices));
  }

  DayScreen(String ip, this.daysOffset, {Key? key}) : super(key: key) {
    loadData(ip);
  }

  String stateString(int state) {
    switch (state) {
      case 1:
        return "Sees";
      case 0:
        return "Väljas";
      default:
        return "Teadmata";
    }
  }

  List<DataRow> getRowsToday(DaySummary summary) {
    final rows = <DataRow>[];
    final now = DateTime.now().add(Duration(days: daysOffset));
    final marketDayStart =
        TZDateTime(marketTimeZone(), now.year, now.month, now.day);

    for (var hour = 0; hour < 24; hour++) {
      final offsetHour = marketDayStart.add(Duration(hours: hour));
      final localHour = TZDateTime.from(offsetHour, localTimeZone()).hour;

      final stateRows = summary.powerStates
          .where((row) => row.momentUTC == offsetHour.toUtc())
          .toList();
      final state = stateRows.isNotEmpty ? stateRows[0].state : -1;

      final rowColor = state == 1 ? Colors.green[300] : Colors.red[300];
      final accentColor = state == 1 ? Colors.green[400] : Colors.red[400];

      var priceString = "-";
      final priceRows = summary.priceCells
          .where((row) => row.momentUTC == offsetHour.toUtc())
          .toList();
      priceString = priceRows.isNotEmpty
          ? priceRows[0].total().toStringAsFixed(2)
          : priceString;

      rows.add(DataRow(color: MaterialStateProperty.all(rowColor), cells: [
        DataCell(Text(
          "$localHour",
          style: rowStyle,
        )),
        DataCell(Text(
          stateString(state),
          style: rowStyleBold,
        )),
        DataCell(Container(
            width: double.infinity,
            decoration: BoxDecoration(
                color: accentColor,
                borderRadius: const BorderRadius.all(Radius.circular(45))),
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Text(
                priceString,
                style: rowStyle,
              ),
            )))
      ]));
    }

    return rows;
  }

  @override
  Widget build(BuildContext context) {
    final rows = <DataRow>[];

    return StreamBuilder<DaySummary>(
        stream: _daySummaryStream.stream,
        builder: (context, snapshot) {
          print(snapshot.hasData);
          final rows =
              snapshot.hasData ? getRowsToday(snapshot.data!) : <DataRow>[];

          final marketDay = marketToday();
          final day = marketDay.day;
          final month = getMonthName(marketDay.month).toUpperCase();
          final year = marketDay.year;

          return Column(
            children: [
              Center(
                child: Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: Text(
                    "$day. $month $year",
                    style: GoogleFonts.secularOne(fontSize: 24),
                  ),
                ),
              ),
              Expanded(
                child: SingleChildScrollView(
                  child: SizedBox(
                    width: double.infinity,
                    child: DataTable(columns: const [
                      DataColumn(label: Text("Tund")),
                      DataColumn(label: Text("Seisund")),
                      DataColumn(label: Text("€/MWh sh. tariif"))
                    ], rows: rows),
                  ),
                ),
              ),
            ],
          );
        });
  }
}
