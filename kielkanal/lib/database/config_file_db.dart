import 'package:kielkanal/config_controller/config_file.dart';
import 'package:kielkanal/database/connection.dart';
import 'package:postgres/postgres.dart';

Future<ConfigFile?> fetchConfigFileFromDatabase(String ip) async {
  const tname = "day_configurations";
  final connection = await newDatabaseConnection(ip);
  final results = await connection.mappedResultsQuery(
      "SELECT $tname.toml FROM $tname WHERE known_broken = false ORDER BY id DESC LIMIT 1");
  if(results.isEmpty) {
    print("Result empty. Using sample!");
    return getSample();
  } else {
    final tomlText = results.first[tname]?["toml"];
    if(tomlText is String) {
      print(tomlText);
      return ConfigFile.fromString(tomlText);
    } else {
      return null;
    }
  }
}
