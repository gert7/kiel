import 'package:flutter/material.dart';
import 'package:kielkanal/config_controller/base.dart';
import 'package:kielkanal/config_controller/config_controller.dart';
import 'package:kielkanal/config_controller/strategy.dart';
import 'package:provider/provider.dart';

enum KielFormType {
  base,
  strategy,
}

class KielForm extends StatelessWidget {
  final int dayNumber;
  final KielFormType rubric;

  const KielForm(this.dayNumber, this.rubric, {Key? key}) : super(key: key);

  static const bases = {
    BaseMode.Tariff: "Tariif",
    BaseMode.AlwaysOn: "Alati sees",
    BaseMode.AlwaysOff: "Alati väljas"
  };

  static const strategies = {
    StrategyMode.None: "Puudub",
    StrategyMode.Smart: "Jaotatud",
    StrategyMode.Limit: "Limiit"
  };

  static const padding = 16.0;

  Widget strategyDropDown(BuildContext context, ControllerDay day) {
    final value = day.strategyMode;
    final textTheme = Theme.of(context).textTheme.titleLarge;

    return Row(
      children: [
        Expanded(
            child: Padding(
              padding: const EdgeInsets.all(padding),
              child: Text(
                "Strateegia",
                style: textTheme,
              ),
            )),
        Expanded(
            child: DropdownButton<StrategyMode>(
              value: value,
              style: textTheme,
              items: strategies
                  .map((key, value) {
                return MapEntry(
                    key,
                    DropdownMenuItem(
                      value: key,
                      child: Text(value),
                    ));
              })
                  .values
                  .toList(),
              onChanged: (StrategyMode? newValue) => day.selectStrategy(newValue!),
            ))
      ],
    );
  }

  Widget baseDropDown(BuildContext context, ControllerDay day) {
    final value = day.baseMode;
    final textTheme = Theme.of(context).textTheme.titleLarge;

    return Row(
      children: [
        Expanded(
            child: Padding(
              padding: const EdgeInsets.all(padding),
              child: Text(
                "Alus",
                style: textTheme,
              ),
            )),
        Expanded(
            child: DropdownButton<BaseMode>(
              value: value,
              style: textTheme,
              items: bases
                  .map((key, value) {
                return MapEntry(
                    key,
                    DropdownMenuItem(
                      value: key,
                      child: Text(value),
                    ));
              })
                  .values
                  .toList(),
              onChanged: (BaseMode? newValue) => day.selectBase(newValue!),
            ))
      ],
    );
  }

  Widget textLine(BuildContext context, ConfigControllerTextInput input) {
    final valid = input.isValid();
    final errorColor = Colors.red[300] ?? Colors.red;
    final color = valid ? Colors.white : errorColor;

    return Row(
      children: [
        Expanded(
            child: Padding(
              padding: const EdgeInsets.all(padding),
              child: Text(
          input.schema.prettyName,
          style: Theme.of(context).textTheme.titleLarge,
        ),
            )),
        Expanded(
            child: ColoredBox(
          color: color,
          child: TextField(
            maxLength: input.textInput.characterLimit(),
            controller: input.controller,
            inputFormatters: input.textInput.getFormatters(),
            style: Theme.of(context).textTheme.titleLarge,
          ),
        ))
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    List<ConfigControllerInput> list = [];

    return Consumer<ConfigController>(builder: (context, controller, child) {
      final rows = <Widget>[];
      final day = controller.days[dayNumber];
      print(rows);

      if (rubric == KielFormType.base) {
        list = day.baseItems;
        rows.add(baseDropDown(context, day));
      } else if (rubric == KielFormType.strategy) {
        list = day.strategyItems;
        rows.add(strategyDropDown(context, day));
      }

      print(list);

      for (final input in list) {
        if (input is ConfigControllerTextInput) {
          rows.add(textLine(context, input));
        }
      }

      print(rows);

      return Column(
        children: rows,
      );
    });
  }
}
