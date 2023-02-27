import 'package:flutter/foundation.dart';
import 'package:kielkanal/database/connection.dart';

class PowerStateDB {
  static const tname = "power_states";

  final DateTime momentUTC;
  final int state;

  PowerStateDB(this.momentUTC, this.state);

  static Future<List<PowerStateDB>> getDay(
      String ip, DateTime localDate) async {
    final startUTC = localDate.toUtc().toIso8601String();
    final endUTC =
        localDate.add(const Duration(days: 1)).toUtc().toIso8601String();
    final connection = await newDatabaseConnection(ip);
    final results = await connection.mappedResultsQuery(
        "SELECT * FROM $tname WHERE moment_utc >= @startTime AND moment_utc < @endTime ORDER BY id DESC",
        substitutionValues: {"startTime": startUTC, "endTime": endUTC});
    debugPrint("start time: $startUTC end time: $endUTC");

    return results.map<PowerStateDB>((jRow) {
      final row = jRow[tname]!;
      return PowerStateDB(row["moment_utc"], row["state"]);
    }).toList();
  }
}

class PriceCellDB {
  static const tname = "price_cells";

  final double priceMWh;
  final DateTime momentUTC;
  final double? tariff;

  PriceCellDB(this.priceMWh, this.momentUTC, this.tariff);

  double total() {
    final tariffValue = tariff ?? 0.0;
    return priceMWh + tariffValue;
  }

  static Future<List<PriceCellDB>> getDay(String ip, DateTime localDate) async {
    final startUTC = localDate.toUtc().toIso8601String();
    final endUTC =
        localDate.add(const Duration(days: 1)).toUtc().toIso8601String();
    final connection = await newDatabaseConnection(ip);
    final results = await connection.mappedResultsQuery(
        "SELECT * FROM $tname WHERE moment_utc >= @startTime AND moment_utc < @endTime ORDER BY id DESC",
        substitutionValues: {"startTime": startUTC, "endTime": endUTC});

    debugPrint("Number of results for prices: ${results.length}");

    return results.map<PriceCellDB>((jRow) {
      final row = jRow[tname]!;
      final price = double.parse(row["price_mwh"]);
      final tariff =
          row["tariff_mwh"] != null ? double.parse(row["tariff_mwh"]) : null;
      return PriceCellDB(price, row["moment_utc"], tariff);
    }).toList();
  }
}

class DaySummary {
  final List<PowerStateDB> powerStates;
  final List<PriceCellDB> priceCells;

  DaySummary(this.powerStates, this.priceCells);
}

class TodayTomorrowSummary {
  final List<PowerStateDB> powerStatesToday;
  final List<PriceCellDB> priceCellsToday;
  final List<PowerStateDB> powerStatesTomorrow;
  final List<PriceCellDB> priceCellsTomorrow;

  TodayTomorrowSummary(this.powerStatesToday, this.priceCellsToday,
      this.powerStatesTomorrow, this.priceCellsTomorrow);

  DaySummary today() {
    return DaySummary(powerStatesToday, priceCellsToday);
  }

  DaySummary tomorrow() {
    return DaySummary(powerStatesTomorrow, priceCellsTomorrow);
  }
}
