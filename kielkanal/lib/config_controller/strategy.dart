// ignore_for_file: constant_identifier_names

import 'package:kielkanal/config_controller/schema.dart';

import 'config_controller.dart';

enum StrategyMode {
  None,
  Limit,
  Smart,
}

abstract class Strategy {
  final StrategyMode mode;
  final Map<String, dynamic> map;

  Strategy(this.mode, this.map);

  Strategy.withoutMap(this.mode) : map = {};

  Map toMap() {
    return {"mode": mode.name};
  }
}

class LimitStrategy extends Strategy {
  static const limitMWhKey = "limit_mwh";

  static const schema = <SchemaItem>[
    SchemaItem(limitMWhKey, EMWhInput(), "Limiit (€/MWh)")
  ];

  LimitStrategy(map) : super(StrategyMode.Limit, map);

  LimitStrategy.fromMap(Map<String, dynamic> map)
      : super(StrategyMode.Limit, map);

  LimitStrategy.fromControllerInputs(List<ConfigControllerInput> cciList)
      : super.withoutMap(StrategyMode.Limit) {
    for (final cci in cciList) {
      if (cci is ConfigControllerTextInput &&
          cci.schema.tomlName == limitMWhKey) {
        map[limitMWhKey] = EMWhInput.getDouble(cci.controller.text);
      }
    }
  }

  @override
  Map toMap() {
    return {"mode": mode.name, limitMWhKey: map[limitMWhKey]};
  }
}

class SmartStrategy extends Strategy {
  static const hourBudgetKey = "hour_budget";
  static const morningHoursKey = "morning_hours";
  static const hardLimitKey = "hard_limit_mwh";

  static const schema = <SchemaItem>[
    SchemaItem(hourBudgetKey, HourInput(), "Tundide kogus"),
    SchemaItem(
        morningHoursKey, HourInput(hourLimit: 7), "Hommikutundide kogus"),
    SchemaItem(hardLimitKey, EMWhInput(), "Hinnalimiit sh. tariif")
  ];

  SmartStrategy(map) : super(StrategyMode.Smart, map);

  SmartStrategy.fromMap(Map<String, dynamic> map)
      : super(StrategyMode.Smart, map);

  SmartStrategy.fromControllerInputs(List<ConfigControllerInput> cciList)
      : super.withoutMap(StrategyMode.Smart) {
    for (final cci in cciList) {
      if (cci is ConfigControllerTextInput) {
        switch (cci.schema.tomlName) {
          case hourBudgetKey:
            map[hourBudgetKey] = HourInput.getInt(cci.controller.text);
            break;
          case morningHoursKey:
            map[morningHoursKey] = HourInput.getInt(cci.controller.text);
            break;
          case hardLimitKey:
            map[hardLimitKey] = EMWhInput.getDouble(cci.controller.text);
            break;
        }
      }
    }
  }

  @override
  Map toMap() {
    return {
      "mode": mode.name,
      hourBudgetKey: map[hourBudgetKey],
      morningHoursKey: map[morningHoursKey],
      hardLimitKey: map[hardLimitKey]
    };
  }
}

Strategy? strategyFromMap(Map<String, dynamic> map) {
  switch (StrategyMode.values.byName(map["mode"])) {
    case StrategyMode.Limit:
      return LimitStrategy(map);
    case StrategyMode.Smart:
      return SmartStrategy(map);
    case StrategyMode.None:
      return null;
  }
}

Strategy? strategyFromInputs(
    StrategyMode? mode, List<ConfigControllerInput> cciList) {
  if (mode == StrategyMode.Limit) {
    return LimitStrategy.fromControllerInputs(cciList);
  } else if (mode == StrategyMode.Smart) {
    return SmartStrategy.fromControllerInputs(cciList);
  } else {
    return null;
  }
}

List<SchemaItem>? getSchemaByStrategyType(StrategyMode? strategyMode) {
  if (strategyMode == StrategyMode.Limit) {
    return LimitStrategy.schema;
  } else if (strategyMode == StrategyMode.Smart) {
    return SmartStrategy.schema;
  } else {
    return null;
  }
}
