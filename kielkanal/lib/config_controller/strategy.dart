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
  Map<String, dynamic>? map;

  Strategy(this.mode, this.map);

  Strategy.withoutMap(this.mode) : map = null;

  Map toMap() {
    return {"mode": mode.string()};
  }
}

class LimitStrategy extends Strategy {
  late final double limit_mwh;
  static const limitMWhKey = "limit_mwh";

  LimitStrategy(this.limit_mwh, map) : super(StrategyMode.Limit, map);

  LimitStrategy.fromMap(Map<String, dynamic> map)
      : limit_mwh = map[limitMWhKey],
        super(StrategyMode.Limit, map);

  LimitStrategy.fromControllerInputs(List<ConfigControllerInput> cciList)
      : super.withoutMap(StrategyMode.Limit) {
    for (final cci in cciList) {
      if (cci is ConfigControllerTextInput &&
          cci.schema.tomlName == limitMWhKey) {
        limit_mwh = EMWhInput.getDouble(cci.controller.text);
      }
    }
  }

  @override
  Map toMap() {
    return {"mode": mode.string(), limitMWhKey: limit_mwh};
  }

  static List<SchemaItem> getSchema() {
    return [SchemaItem("limit_mwh", EMWhInput(), "Limiit (€/MWh)")];
  }
}

class SmartStrategy extends Strategy {
  late final int hour_budget;
  static const hourBudgetKey = "hour_budget";

  SmartStrategy(this.hour_budget, map) : super(StrategyMode.Smart, map);

  SmartStrategy.fromMap(Map<String, dynamic> map)
      : hour_budget = map[hourBudgetKey],
        super(StrategyMode.Smart, map);

  SmartStrategy.fromControllerInputs(List<ConfigControllerInput> cciList)
      : super.withoutMap(StrategyMode.Smart) {
    for (final cci in cciList) {
      if (cci is ConfigControllerTextInput &&
          cci.schema.tomlName == hourBudgetKey) {
        hour_budget = HourInput.getInt(cci.controller.text);
      }
    }
  }

  @override
  Map toMap() {
    return {
      "mode": mode.string(),
      hourBudgetKey: hour_budget,
    };
  }

  static List<SchemaItem> getSchema() {
    return [SchemaItem("hour_budget", HourInput(), "Tundide kogus")];
  }
}

Strategy? strategyFromMap(Map<String, dynamic> map) {
  switch (strategyTypeFromString(map["mode"])) {
    case StrategyMode.Limit:
      return LimitStrategy(map["limit_mwh"], map);
    case StrategyMode.Smart:
      return SmartStrategy(map["hour_budget"], map);
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
