enum BaseType {
  AlwaysOn,
  AlwaysOff,
  Tariff,
}

extension on BaseType {
  String string() {
    if (this == BaseType.AlwaysOn) {
      return "AlwaysOn";
    } else if (this == BaseType.AlwaysOff) {
      return "AlwaysOff";
    } else {
      return "Tariff";
    }
  }
}

BaseType baseTypeFromString(String s) {
  if (s == "AlwaysOn") {
    return BaseType.AlwaysOn;
  } else if (s == "AlwaysOff") {
    return BaseType.AlwaysOff;
  } else {
    return BaseType.Tariff;
  }
}

class Base {
  final BaseType mode;

  Base(this.mode);

  Base.fromMap(Map<String, dynamic> map)
      : mode = baseTypeFromString(map["mode"]);

  Map toMap() {
    return {"mode": mode.string()};
  }
}
