import 'package:kielkanal/config_controller/base.dart';
import 'package:kielkanal/config_controller/sample.dart';
import 'package:kielkanal/config_controller/strategy.dart';
import 'package:toml/toml.dart';

const dayNamesEnglish = [
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
  "Sunday"
];

List<int> intListFromDynamicList(List<dynamic> inList) {
  final list = <int>[];
  for (final i in inList) {
    if (i is int) {
      list.add(i);
    }
  }
  return list;
}

List<int>? intListGiven(Map<String, dynamic> map, String key) {
  final list = map[key] != null ? intListFromDynamicList(map[key]) : null;
  return list;
}

class ConfigDay {
  final List<int>? hoursAlwaysOn;
  final List<int>? hoursAlwaysOff;
  final Base? base;
  final Strategy? strategy;

  ConfigDay(this.hoursAlwaysOn, this.hoursAlwaysOff, this.base, this.strategy);

  static ConfigDay fromMap(Map<String, dynamic> map) {
    final hoursAlwaysOn = intListGiven(map, "hours_always_on");
    final hoursAlwaysOff = intListGiven(map, "hours_always_off");
    final base = map["base"] != null ? Base.fromMap(map["base"]) : null;
    final strategy =
        map["strategy"] != null ? strategyFromMap(map["strategy"]) : null;
    return ConfigDay(hoursAlwaysOn, hoursAlwaysOff, base, strategy);
  }

  Map toMap() {
    final m = {
      "hours_always_on": hoursAlwaysOn,
      "hours_always_off": hoursAlwaysOff,
      "base": base?.toMap(),
      "strategy": strategy?.toMap(),
    };
    m.removeWhere((key, value) => value == null);
    return m;
  }
}

class ConfigFile {
  final List<ConfigDay> days = [];

  ConfigFile.fromMap(Map<String, dynamic> map) {
    for (final dayName in dayNamesEnglish) {
      final day = ConfigDay.fromMap(map[dayName.toLowerCase()]);
      days.add(day);
    }
  }

  Map toMap() {
    final map = {};
    for(int i = 0; i < days.length; i++) {
      map[dayNamesEnglish[i]] = days[i].toMap();
    }
    return map;
  }
}

ConfigFile getSample() {
  final decoded = TomlDocument.parse(sampleTOML).toMap();
  return ConfigFile.fromMap(decoded);
}
