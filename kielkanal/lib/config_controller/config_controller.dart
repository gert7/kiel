import 'package:flutter/material.dart';
import 'package:kielkanal/config_controller/base.dart';
import 'package:kielkanal/config_controller/schema.dart';
import 'package:kielkanal/config_controller/strategy.dart';

import 'config_file.dart';

abstract class ConfigControllerInput {
  final SchemaItem schema;

  ConfigControllerInput(this.schema);

  ConfigControllerInput clone(
      ConfigControllerInput original, Function() notify);

  bool isValid();
}

class ConfigControllerTextInput extends ConfigControllerInput {
  final KielTextInput textInput;

  final TextEditingController controller;

  bool _isValid = false;

  ConfigControllerTextInput(SchemaItem schema, this.textInput, this.controller)
      : super(schema) {
    _validate();
    controller.addListener(_validate);
  }

  void _validate() {
    _isValid = textInput.isValid(controller.text);
  }

  @override
  bool isValid() => _isValid;

  @override
  ConfigControllerInput clone(
      ConfigControllerInput original, Function() notify) {
    var textValue = "";
    if (original is ConfigControllerTextInput) {
      textValue = original.controller.text;
    }
    final controller =
        TextEditingController.fromValue(TextEditingValue(text: textValue));
    controller.addListener(() {
      print(controller.text);
      print("reckoning");
      notify();
    });
    print("cloning $textInput");
    return ConfigControllerTextInput(schema, textInput, controller);
  }
}

class ControllerDay extends ChangeNotifier {
  ConfigDay day;
  List<int> hoursAlwaysOn;
  List<int> hoursAlwaysOff;

  late BaseMode baseMode;
  List<ConfigControllerInput> baseItems = [];

  late StrategyMode strategyMode;
  List<ConfigControllerInput> strategyItems = [];

  void populateStrategySchema(StrategyMode? mode, bool initial) {
    strategyItems = [];

    final schema = getSchemaByStrategyType(strategyMode);
    if (schema != null) {
      for (final item in schema) {
        final input = item.input;
        if (input is KielTextInput) {
          String textValue = "";
          if (initial) {
            final value = day.strategy?.map[item.tomlName];
            if (value is double) {
              textValue = value.toString().replaceAll(".", ",");
            } else if (value is int) {
              textValue = value.toString();
            }
          }
          final controller = TextEditingController.fromValue(
              TextEditingValue(text: textValue));
          controller.addListener(notifyListeners);
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

    populateStrategySchema(strategyMode, true);
  }

  void cycleHour(int hour) {
    if (hour >= 0 && hour <= 23) {
      if (hoursAlwaysOn.contains(hour)) {
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

  bool isValid() {
    final baseItemsInvalid = baseItems.any((element) => !element.isValid());
    final strategyItemsInvalid = strategyItems.any((element) => !element.isValid());
    return !(baseItemsInvalid || strategyItemsInvalid);
  }

  ControllerDay.clone(ControllerDay original)
      : day = original.day,
        hoursAlwaysOn = original.hoursAlwaysOn.map((e) => e).toList(),
        hoursAlwaysOff = original.hoursAlwaysOff.map((e) => e).toList(),
        baseMode = original.baseMode,
        strategyMode = original.strategyMode,
        // TODO: change if base items ever get options
        baseItems = [] {
    strategyItems =
        original.strategyItems.map((e) => e.clone(e, notifyListeners)).toList();
  }
}

class ConfigController extends ChangeNotifier {
  final List<ControllerDay> _days = [];

  ControllerDay day(int index) {
    return _days[index];
  }

  void selectBase(int day, BaseMode mode) {
    _days[day].baseMode = mode;
    notifyListeners();
  }

  void selectStrategy(int day, StrategyMode mode) {
    _days[day].strategyMode = mode;
    _days[day].populateStrategySchema(mode, false);
    notifyListeners();
  }

  void cycleHour(int day, int hour) {
    _days[day].cycleHour(hour);
  }

  ConfigController.fromConfigFile(ConfigFile configFile) {
    for (final day in configFile.days) {
      final controllerDay = ControllerDay.getDayFromConfig(day);
      controllerDay.addListener(notifyListeners);
      _days.add(controllerDay);
    }
  }

  void copyDayOver(int fromDay, int toDay) {
    _days[toDay] = ControllerDay.clone(_days[fromDay]);
    _days[toDay].addListener(notifyListeners);
    notifyListeners();
  }

  bool isValid() {
    return !_days.any((element) => !element.isValid());
  }

  // static ConfigController fromSampleConfigFile() {
  //   return ConfigController.fromConfigFile(getSample());
  // }

  ConfigFile toConfigFile() {
    final newDays = _days.map((cDay) {
      final base = Base(cDay.baseMode);
      final strategy =
          strategyFromInputs(cDay.strategyMode, cDay.strategyItems);
      return ConfigDay(cDay.hoursAlwaysOn, cDay.hoursAlwaysOff, base, strategy);
    }).toList();
    return ConfigFile(newDays);
  }

  Map toMap() {
    return toConfigFile().toMap();
  }
}
