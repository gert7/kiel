enum BaseMode {
  AlwaysOn,
  AlwaysOff,
  Tariff,
}

extension on BaseMode {
  String string() {
    if (this == BaseMode.AlwaysOn) {
      return "AlwaysOn";
    } else if (this == BaseMode.AlwaysOff) {
      return "AlwaysOff";
    } else {
      return "Tariff";
    }
  }
}

BaseMode baseTypeFromString(String s) {
  if (s == "AlwaysOn") {
    return BaseMode.AlwaysOn;
  } else if (s == "AlwaysOff") {
    return BaseMode.AlwaysOff;
  } else {
    return BaseMode.Tariff;
  }
}

class Base {
  final BaseMode mode;

  Base(this.mode);

  Base.fromMap(Map<String, dynamic> map)
      : mode = baseTypeFromString(map["mode"]);

  Map toMap() {
    return {"mode": mode.string()};
  }
}
