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

  ConfigDay(
      this.hoursAlwaysOn, this.hoursAlwaysOff, this.base, this.strategy);

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
  final ConfigDay monday;
  final ConfigDay tuesday;
  final ConfigDay wednesday;
  final ConfigDay thursday;
  final ConfigDay friday;
  final ConfigDay saturday;
  final ConfigDay sunday;

  ConfigFile(this.monday, this.tuesday, this.wednesday, this.thursday,
      this.friday, this.saturday, this.sunday);

  ConfigFile.fromMap(Map<String, dynamic> map)
      : monday = ConfigDay.fromMap(map["monday"]),
        tuesday = ConfigDay.fromMap(map["tuesday"]),
        wednesday = ConfigDay.fromMap(map["wednesday"]),
        thursday = ConfigDay.fromMap(map["thursday"]),
        friday = ConfigDay.fromMap(map["friday"]),
        saturday = ConfigDay.fromMap(map["saturday"]),
        sunday = ConfigDay.fromMap(map["sunday"]);

  Map toMap() {
    return {
      "monday": monday.toMap(),
      "tuesday": tuesday.toMap(),
      "wednesday": wednesday.toMap(),
      "thursday": thursday.toMap(),
      "friday": friday.toMap(),
      "saturday": saturday.toMap(),
      "sunday": sunday.toMap(),
    };
  }
}

ConfigFile getSample() {
  final decoded = TomlDocument.parse(sampleTOML).toMap();
  return ConfigFile.fromMap(decoded);
}
