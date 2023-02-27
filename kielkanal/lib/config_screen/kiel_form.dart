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

  Widget rubricDropDown<M>(BuildContext context, int dayIndex, M value,
      String title, Function(int, M) change, Map<M, String> enumMap) {
    final textTheme = Theme.of(context).textTheme.titleLarge;

    return Row(
      children: [
        Expanded(
            child: Padding(
          padding: const EdgeInsets.all(padding),
          child: Text(
            title,
            style: textTheme,
          ),
        )),
        Expanded(
            child: DropdownButton<M>(
          value: value,
          style: textTheme,
          items: enumMap
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
          onChanged: (M? newValue) {
            if (newValue != null) change(dayIndex, newValue);
          },
        ))
      ],
    );
  }

  Widget textLine(BuildContext context, ConfigControllerTextInput input) {
    final valid = input.isValid();
    final errorColor = Colors.red[300] ?? Colors.red;
    final color = valid ? Colors.transparent : errorColor;

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
    return SingleChildScrollView(
      child: Consumer<ConfigController>(builder: (context, controller, child) {
        List<ConfigControllerInput> list = [];
        debugPrint("Rebuilding KielForm");
        final day = controller.day(dayNumber);
        final rows = <Widget>[];

        if (rubric == KielFormType.base) {
          list = day.baseItems;
          rows.add(rubricDropDown(context, dayNumber, day.baseMode, "Alused",
              controller.selectBase, bases));
        } else if (rubric == KielFormType.strategy) {
          list = day.strategyItems;
          rows.add(rubricDropDown(context, dayNumber, day.strategyMode,
              "Strateegia", controller.selectStrategy, strategies));
        }

        for (final input in list) {
          if (input is ConfigControllerTextInput) {
            rows.add(textLine(context, input));
          }
        }

        debugPrint("$rows");

        return Column(
          children: rows,
        );
      }),
    );
  }
}
