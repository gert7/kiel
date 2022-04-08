import 'package:flutter/material.dart';

class WeeklyOverrideWidget extends StatelessWidget {
  final int dayNumber;
  const WeeklyOverrideWidget(this.dayNumber, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final children = <Widget>[];
    for (var i = 0; i < 24; i++) {
      children.add(
          Card(child: Text(i.toString()),)
      );
    }

    return GridView.count(
      padding: const EdgeInsets.all(8.0),
      crossAxisCount: 4,
      children: children,
    );
  }
}
