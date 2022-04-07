import 'package:flutter/material.dart';
import 'package:kielkanal/config_controller/base.dart';
import 'package:kielkanal/config_controller/schema.dart';
import 'package:kielkanal/config_controller/strategy.dart';

import 'config_file.dart';

abstract class ConfigControllerInput {
  final SchemaItem schema;

  ConfigControllerInput(this.schema);
}

class ConfigControllerTextInput extends ConfigControllerInput {
  final KielTextInput textInput;

  final TextEditingController controller;

  ConfigControllerTextInput(SchemaItem schema, this.textInput, this.controller)
      : super(schema);

  bool isValid() {
    print("Validating on: ${controller.text}");
    return textInput.isValid(controller.text);
  }
}

class ControllerDay extends ChangeNotifier {
  List<int> hoursAlwaysOn;
  List<int> hoursAlwaysOff;

  BaseType? baseMode;
  List<ConfigControllerInput>? baseItems;

  StrategyType? strategyMode;
  List<SchemaItem>? strategySchema;
  List<ConfigControllerInput>? strategyItems;

  ControllerDay(this.baseMode, this.baseItems, this.hoursAlwaysOff,
      this.hoursAlwaysOn);

  ControllerDay.getDayFromConfig(ConfigDay day)
      : hoursAlwaysOn = day.hoursAlwaysOn ?? [],
        hoursAlwaysOff = day.hoursAlwaysOff ?? [] {
    baseMode = day.base?.mode ?? BaseType.Tariff;
    baseItems = [];

    strategyMode = day.strategy?.mode;
    strategyItems = [];

    strategySchema = getSchemaByStrategyType(strategyMode);
    final schema = strategySchema;
    if (schema != null) {
      for (final item in schema) {
        final input = item.input;
        if (input is KielTextInput) {
          final value = day.strategy?.map[item.tomlName];
          String? textValue;
          if(value is double) {
            textValue = value.toString().replaceAll(".", ",");
          } else if (value is int) {
            textValue = value.toString();
          }
          final controller = TextEditingController.fromValue(
              TextEditingValue(text: textValue ?? ""));
          controller.addListener(() {
            final String text = controller.text;
            print(text);
            notifyListeners();
          });
          final cci = ConfigControllerTextInput(item, input, controller);
          strategyItems?.add(cci);
        }
      }
    }
  }
}

class ConfigController extends ChangeNotifier {
  List<ControllerDay> days = [];

  ConfigController.fromConfigFile(ConfigFile configFile) {
    for(final day in configFile.days) {
      final controllerDay = ControllerDay.getDayFromConfig(day);
      controllerDay.addListener(() {
        notifyListeners();
      });
      days.add(controllerDay);
    }
  }

  static ConfigController fromSampleConfigFile() {
    return ConfigController.fromConfigFile(getSample());
  }

  @override
  void dispose() {
    super.dispose();
  }
}

