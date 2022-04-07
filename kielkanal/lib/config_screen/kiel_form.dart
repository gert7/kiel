import 'package:flutter/material.dart';
import 'package:kielkanal/config_controller/config_controller.dart';
import 'package:provider/provider.dart';

enum KielFormType {
  base,
  strategy,
}

class KielForm extends StatelessWidget {
  final int day;
  final KielFormType rubric;

  const KielForm(this.day, this.rubric, {Key? key}) : super(key: key);

  Widget textLine(BuildContext context, ConfigControllerTextInput input) {
    final valid = input.isValid();
    final errorColor = Colors.red[300] ?? Colors.red;
    final color = valid ? Colors.white : errorColor;

    return Row(
      children: [
        Expanded(
            child: Text(
              input.schema.prettyName,
              style: Theme.of(context).textTheme.headline5,
            )),
        Expanded(
            child: ColoredBox(
              color: color,
              child: TextField(
          controller: input.controller,
          inputFormatters: input.textInput.getFormatters(),
          style: Theme.of(context).textTheme.headline5,
        ),
            ))
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    List<ConfigControllerInput>? list;

    return Consumer<ConfigController>(builder: (context, controller, child) {
      if (rubric == KielFormType.strategy) {
        list = controller.days[day].strategyItems;
      } else if (rubric == KielFormType.base) {
        list = controller.days[day].baseItems;
      }

      final rows = <Widget>[];

      if (list != null) {
        for (final input in list!) {
          if (input is ConfigControllerTextInput) {
            rows.add(textLine(context, input));
          }
        }
      }

      return Column(
        children: rows,
      );
    });
  }
}
