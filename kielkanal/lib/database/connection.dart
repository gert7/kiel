import 'package:postgres/postgres.dart';

Future<PostgreSQLConnection> newDatabaseConnection(String ip) async {
  final connection = PostgreSQLConnection(ip, 5432, "kiel",
      username: "kiel", password: "kiel");
  await connection.open();
  return connection;
}
