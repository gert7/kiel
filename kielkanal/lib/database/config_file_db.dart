import 'dart:convert';
import 'dart:io';

import 'package:kielkanal/config_controller/config_file.dart';
import 'package:kielkanal/database/connection.dart';
import 'package:postgres/postgres.dart';
import 'package:toml/toml.dart';

class ConfigFileDB {
  static const tname = "day_configurations";
}

Future<ConfigFile?> fetchConfigFileFromDatabase(String ip) async {
  const tname = ConfigFileDB.tname;
  final connection = await newDatabaseConnection(ip);
  final results = await connection.mappedResultsQuery(
      "SELECT $tname.toml FROM $tname WHERE known_broken = false AND tried = true ORDER BY id DESC LIMIT 1");
  if (results.isEmpty) {
    print("Result empty. Using sample!");
    return getSample();
  } else {
    final tomlText = results.first[tname]?["toml"];
    if (tomlText is String) {
      print(tomlText);
      return ConfigFile.fromString(tomlText);
    } else {
      return null;
    }
  }
}

/// TODO: Should also ping up the server!
Future<bool> sendConfigFileToDatabase(String ip, TomlDocument toml) async {
  const tname = ConfigFileDB.tname;
  final tomlString = toml.toString();
  final connection = await newDatabaseConnection(ip);
  final results = await connection.query(
      "INSERT INTO $tname (toml, known_broken, tried) VALUES (@tString, FALSE, FALSE)",
      substitutionValues: {"tString": tomlString});
  print(results);

  final client = HttpClient();
  try {
    final request = await client.get(ip, 8196, "hour");
    final response = await request.close();
    final stringData = await response.transform(utf8.decoder).join();
    print(stringData);

    print("hour gotten");
  } catch (e) {
    print("Erroir");
    return false;
  }

  return true;
}
