import 'package:decimal/decimal.dart';
import 'package:kielkanal/config_controller/schema.dart';

enum StrategyType {
  Limit,
  Smart,
}

extension on StrategyType {
  String string() {
    if (this == StrategyType.Limit) {
      return "Limit";
    } else {
      return "Smart";
    }
  }
}

StrategyType strategyTypeFromString(String s) {
  if (s == "Limit") {
    return StrategyType.Limit;
  } else {
    return StrategyType.Smart;
  }
}

abstract class Strategy {
  final StrategyType mode;
  final Map<String, dynamic> map;

  Strategy(this.mode, this.map);

  Map toMap() {
    return {"mode": mode.string()};
  }
}

class LimitStrategy extends Strategy {
  final double limit_mwh;

  LimitStrategy(this.limit_mwh, map) : super(StrategyType.Limit, map);

  LimitStrategy.fromMap(Map<String, dynamic> map)
      : limit_mwh = map["limit_mwh"],
        super(StrategyType.Limit, map);

  @override
  Map toMap() {
    return {"mode": mode.string(), "limit_mwh": limit_mwh};
  }

  static List<SchemaItem> getSchema() {
    return [
      SchemaItem("limit_mwh", DecimalInput())
    ];
  }
}

class SmartStrategy extends Strategy {
  final int hour_budget;

  SmartStrategy(this.hour_budget, map) : super(StrategyType.Smart, map);

  SmartStrategy.fromMap(Map<String, dynamic> map)
      : hour_budget = map["hour_budget"],
        super(StrategyType.Smart, map);

  @override
  Map toMap() {
    return {
      "mode": mode.string(),
      "hour_budget": hour_budget,
    };
  }

  static List<SchemaItem> getSchema() {
    return [
      SchemaItem("hour_budget", IntegerInput())
    ];
  }
}

Strategy strategyFromMap(Map<String, dynamic> map) {
  switch (strategyTypeFromString(map["mode"])) {
    case StrategyType.Limit:
      return LimitStrategy(map["limit_mwh"], map);
    case StrategyType.Smart:
      return SmartStrategy(map["hour_budget"], map);
  }
}

List<SchemaItem>? getSchemaByStrategyType(StrategyType? strategyMode) {
  if(strategyMode == StrategyType.Limit) {
    return LimitStrategy.getSchema();
  } else if (strategyMode == StrategyType.Smart) {
    return SmartStrategy.getSchema();
  } else {
    return null;
  }
}
