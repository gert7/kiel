import 'package:kielkanal/config_controller/schema.dart';

import 'config_controller.dart';

enum StrategyMode {
  None,
  Limit,
  Smart,
}

extension on StrategyMode {
  String string() {
    switch (this) {
      case StrategyMode.None:
        return "None";
      case StrategyMode.Limit:
        return "Limit";
      case StrategyMode.Smart:
        return "Smart";
    }
  }
}

StrategyMode strategyTypeFromString(String s) {
  if (s == "Limit") {
    return StrategyMode.Limit;
  } else if (s == "Smart") {
    return StrategyMode.Smart;
  } else {
    return StrategyMode.None;
  }
}

abstract class Strategy {
  final StrategyMode mode;
  final Map<String, dynamic> map;

  Strategy(this.mode, this.map);

  Strategy.withoutMap(this.mode) : map = {};

  Map toMap() {
    return {"mode": mode.string()};
  }
}

class LimitStrategy extends Strategy {
  static const limitMWhKey = "limit_mwh";

  static List<SchemaItem> getSchema() {
    return [SchemaItem(limitMWhKey, EMWhInput(), "Limiit (€/MWh)")];
  }

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
    return {"mode": mode.string(), limitMWhKey: map[limitMWhKey]};
  }
}

class SmartStrategy extends Strategy {
  static const hourBudgetKey = "hour_budget";

  SmartStrategy(map) : super(StrategyMode.Smart, map);

  SmartStrategy.fromMap(Map<String, dynamic> map)
      : super(StrategyMode.Smart, map);

  SmartStrategy.fromControllerInputs(List<ConfigControllerInput> cciList)
      : super.withoutMap(StrategyMode.Smart) {
    for (final cci in cciList) {
      if (cci is ConfigControllerTextInput &&
          cci.schema.tomlName == hourBudgetKey) {
        map[hourBudgetKey] = HourInput.getInt(cci.controller.text);
      }
    }
  }

  @override
  Map toMap() {
    return {
      "mode": mode.string(),
      hourBudgetKey: map[hourBudgetKey],
    };
  }

  static List<SchemaItem> getSchema() {
    return [SchemaItem("hour_budget", HourInput(), "Tundide kogus")];
  }
}

Strategy? strategyFromMap(Map<String, dynamic> map) {
  switch (strategyTypeFromString(map["mode"])) {
    case StrategyMode.Limit:
      return LimitStrategy(map);
    case StrategyMode.Smart:
      return SmartStrategy(map);
    case StrategyMode.None:
      return null;
  }
}

Strategy? strategyFromInputs(StrategyMode? mode, List<ConfigControllerInput> cciList) {
  if(mode == StrategyMode.Limit) {
    return LimitStrategy.fromControllerInputs(cciList);
  } else if (mode == StrategyMode.Smart) {
    return SmartStrategy.fromControllerInputs(cciList);
  } else {
    return null;
  }
}

List<SchemaItem>? getSchemaByStrategyType(StrategyMode? strategyMode) {
  if (strategyMode == StrategyMode.Limit) {
    return LimitStrategy.getSchema();
  } else if (strategyMode == StrategyMode.Smart) {
    return SmartStrategy.getSchema();
  } else {
    return null;
  }
}
