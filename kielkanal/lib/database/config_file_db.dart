import 'package:kielkanal/config_controller/config_file.dart';
import 'package:postgres/postgres.dart';

Future<ConfigFile?> fetchFromDatabase(String ip) async {
  const tname = "day_configurations";
  final connection = PostgreSQLConnection(ip, 5432, "kiel",
      username: "kiel", password: "kiel");
  await connection.open();
  final results = await connection.mappedResultsQuery(
      "SELECT $tname.toml FROM $tname WHERE known_broken = false ORDER BY id DESC LIMIT 1");
  if(results.isEmpty) {
    print("RESULT EMPTY. USING SAMPLE!!");
    return getSample();
  } else {
    final tomlText = results.first[tname]?["toml"];
    if(tomlText is String) {
      return ConfigFile.fromString(tomlText);
    } else {
      return null;
    }
  }
}
