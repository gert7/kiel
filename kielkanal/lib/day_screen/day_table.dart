import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:timezone/timezone.dart';

import '../database/constants.dart';
import '../database/day_summary_db.dart';
import 'day_screen.dart';

class DayScreenTable extends StatelessWidget {
  static const rowStyleBold =
  TextStyle(fontSize: 16, fontWeight: FontWeight.bold);
  static const rowStyle = TextStyle(fontSize: 16);

  final DaySummary summary;
  final DateTime moment;
  const DayScreenTable(this.summary, this.moment, {Key? key}) : super(key: key);

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

  List<Color?> getRowColor(int state) {
    switch(state) {
      case 1: return [Colors.green[300], Colors.green[400]];
      case 0: return [Colors.red[300], Colors.red[400]];
      default: return [Colors.grey[300], Colors.grey[400]];
    }
  }

  List<DataRow> getRows(DaySummary summary, DateTime moment) {
    final rows = <DataRow>[];
    // final now = DateTime.now().add(Duration(days: daysOffset));
    final marketDayStart =
    TZDateTime(marketTimeZone(), moment.year, moment.month, moment.day);

    for (var hour = 0; hour < 24; hour++) {
      final offsetHour = marketDayStart.add(Duration(hours: hour));
      final localHour = TZDateTime.from(offsetHour, localTimeZone()).hour;

      final stateRows = summary.powerStates
          .where((row) => row.momentUTC == offsetHour.toUtc())
          .toList();
      final state = stateRows.isNotEmpty ? stateRows[0].state : -1;

      final colors = getRowColor(state);
      final rowColor = colors[0];
      final accentColor = colors[1];

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
    final rows = getRows(summary, moment);

    final monthName = getMonthName(moment.month).toUpperCase();

    return Column(
      children: [
        Center(
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: Text(
              "${moment.day}. $monthName ${moment.year}",
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
  }
}