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
  ConfigDay day;
  List<int> hoursAlwaysOn;
  List<int> hoursAlwaysOff;

  late BaseMode baseMode;
  List<ConfigControllerInput> baseItems = [];

  late StrategyMode strategyMode;
  List<SchemaItem>? strategySchema;
  List<ConfigControllerInput> strategyItems = [];

  void populateStrategySchema(StrategyMode? mode) {
    strategyItems = [];

    strategySchema = getSchemaByStrategyType(strategyMode);
    final schema = strategySchema;
    print(schema);
    print(strategyMode);
    if (schema != null) {
      for (final item in schema) {
        final input = item.input;
        if (input is KielTextInput) {
          final value = day.strategy?.map[item.tomlName];
          String textValue = "";
          if(value is double) {
            textValue = value.toString().replaceAll(".", ",");
          } else if (value is int) {
            textValue = value.toString();
          }
          final controller = TextEditingController.fromValue(
              TextEditingValue(text: textValue));
          controller.addListener(() {
            final String text = controller.text;
            print(text);
            notifyListeners();
          });
          final cci = ConfigControllerTextInput(item, input, controller);
          strategyItems.add(cci);
        }
      }
    }
  }

  ControllerDay.getDayFromConfig(this.day)
      : hoursAlwaysOn = day.hoursAlwaysOn ?? [],
        hoursAlwaysOff = day.hoursAlwaysOff ?? [] {
    baseMode = day.base?.mode ?? BaseMode.Tariff; // BASE DEFAULT
    baseItems = [];

    strategyMode = day.strategy?.mode ?? StrategyMode.None;

    populateStrategySchema(strategyMode);
  }

  void selectBase(BaseMode mode) {
    baseMode = mode;
    notifyListeners();
  }

  void selectStrategy(StrategyMode mode) {
    print("selecting strategy");
    strategyMode = mode;
    print("populating strategy");
    populateStrategySchema(mode);
    print("notifying");
    notifyListeners();
  }

  void cycleHour(int hour) {
    if (hour >= 0 && hour <= 23) {
      if(hoursAlwaysOn.contains(hour)) {
        hoursAlwaysOn.remove(hour);
        hoursAlwaysOff.add(hour);
      } else if (hoursAlwaysOff.contains(hour)) {
        hoursAlwaysOff.remove(hour);
      } else {
        hoursAlwaysOn.add(hour);
      }
      notifyListeners();
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

  ConfigFile toConfigFile() {
    final newDays = days.map((cDay) {
      final base = Base(cDay.baseMode);
      final strategy = strategyFromInputs(cDay.strategyMode, cDay.strategyItems);
      return ConfigDay(cDay.hoursAlwaysOn, cDay.hoursAlwaysOff, base, strategy);
    }).toList();
    return ConfigFile(newDays);
  }

  Map toMap() {
    return toConfigFile().toMap();
  }
}
