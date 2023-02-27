import 'package:flutter/material.dart';
import 'package:kielkanal/config_controller/config_controller.dart';
import 'package:provider/provider.dart';

class WeeklyOverrideWidget extends StatelessWidget {
  final int dayNumber;

  const WeeklyOverrideWidget(this.dayNumber, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<ConfigController>(
      builder: (context, controller, child) {
        final day = controller.day(dayNumber);
        final bigStyle = Theme.of(context).textTheme.headlineMedium;

        final children = <Widget>[];
        for (var hour = 0; hour < 24; hour++) {
          var color = Colors.transparent;
          if (day.hoursAlwaysOn.contains(hour)) {
            color = Colors.greenAccent;
          } else if (day.hoursAlwaysOff.contains(hour)) {
            color = Colors.redAccent;
          }

          children.add(Card(
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              color: color,
              child: InkWell(
                  onTap: () => controller.cycleHour(dayNumber, hour),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Center(
                          child: Text(
                        "$hour",
                        style: bigStyle,
                      ))
                    ],
                  )),
            ),
          ));
        }

        return GridView.count(
          padding: const EdgeInsets.all(8.0),
          crossAxisCount: 4,
          children: children,
        );
      },
    );
  }
}
