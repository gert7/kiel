import 'package:flutter/material.dart';

class DayScreen extends StatelessWidget {
  const DayScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final rows = <DataRow>[];
    // final rowStyle = Theme.of(context).textTheme.headline5;
    const rowStyleBold = TextStyle(fontSize: 16, fontWeight: FontWeight.bold);
    const rowStyle = TextStyle(fontSize: 16);

    for (var i = 0; i < 24; i++) {
      rows.add(DataRow(color: MaterialStateProperty.all(Colors.green), cells: [
        DataCell(Text("$i", style: rowStyle,)),
        DataCell(Text("Sees", style: rowStyleBold,)),
        DataCell(Container(
          width: double.infinity,
            // color: Colors.blueAccent,
            decoration: BoxDecoration(
              color: Colors.green[200],
                // border: Border.all(),
                borderRadius: const BorderRadius.all(Radius.circular(45))),
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Text("2.48", style: rowStyle,),
            )))
      ]));
    }

    return StreamBuilder<Object>(
      stream: null,
      builder: (context, snapshot) {
        return SingleChildScrollView(
          child: SizedBox(
            width: double.infinity,
            child: DataTable(columns: const [
              DataColumn(label: Text("T")),
              DataColumn(label: Text("Seisund")),
              DataColumn(label: Text("Elektrihind"))
            ], rows: rows),
          ),
        );
      }
    );
  }
}
