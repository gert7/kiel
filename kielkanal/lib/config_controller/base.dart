enum BaseMode {
  AlwaysOn,
  AlwaysOff,
  Tariff,
}

class Base {
  final BaseMode mode;

  Base(this.mode);

  Base.fromMap(Map<String, dynamic> map)
      : mode = BaseMode.values.byName(map["mode"]);

  Map toMap() {
    return {"mode": mode.name};
  }
}
