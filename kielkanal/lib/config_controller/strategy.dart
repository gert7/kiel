import 'package:decimal/decimal.dart';
import 'package:kielkanal/config_controller/schema.dart';

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

  Map toMap() {
    return {"mode": mode.string()};
  }
}

class LimitStrategy extends Strategy {
  final double limit_mwh;

  LimitStrategy(this.limit_mwh, map) : super(StrategyMode.Limit, map);

  LimitStrategy.fromMap(Map<String, dynamic> map)
      : limit_mwh = map["limit_mwh"],
        super(StrategyMode.Limit, map);

  @override
  Map toMap() {
    return {"mode": mode.string(), "limit_mwh": limit_mwh};
  }

  static List<SchemaItem> getSchema() {
    return [
      SchemaItem("limit_mwh", EMWhInput(), "Limiit (€/MWh)")
    ];
  }
}

class SmartStrategy extends Strategy {
  final int hour_budget;

  SmartStrategy(this.hour_budget, map) : super(StrategyMode.Smart, map);

  SmartStrategy.fromMap(Map<String, dynamic> map)
      : hour_budget = map["hour_budget"],
        super(StrategyMode.Smart, map);

  @override
  Map toMap() {
    return {
      "mode": mode.string(),
      "hour_budget": hour_budget,
    };
  }

  static List<SchemaItem> getSchema() {
    return [
      SchemaItem("hour_budget", HourInput(), "Tundide kogus")
    ];
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

List<SchemaItem>? getSchemaByStrategyType(StrategyMode? strategyMode) {
  if(strategyMode == StrategyMode.Limit) {
    return LimitStrategy.getSchema();
  } else if (strategyMode == StrategyMode.Smart) {
    return SmartStrategy.getSchema();
  } else {
    return null;
  }
}
