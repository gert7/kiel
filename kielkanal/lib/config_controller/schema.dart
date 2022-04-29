import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:kielkanal/formatters.dart';

abstract class SchemaInput {
  const SchemaInput();
}

class SchemaItem {
  final String tomlName;

  final String prettyName;

  final SchemaInput input;

  const SchemaItem(this.tomlName, this.input, this.prettyName);
}

abstract class KielTextInput extends SchemaInput {
  List<TextInputFormatter> getFormatters();

  bool allowText(String s);

  bool isValid(String t);

  int? characterLimit();

  const KielTextInput();
}

class EMWhInput extends KielTextInput {
  const EMWhInput();

  @override
  List<TextInputFormatter> getFormatters() {
    return [
      EMWhValidator().getFormatter(),
      getDecimalFormatter()
    ];
  }

  @override
  bool allowText(String s) => EMWhValidator().allowText(s);

  @override
  bool isValid(String t) {
    final ta = t.replaceAll(",", ".");
    if(!allowText(ta)) { return false; }
    if(ta.isEmpty || !ta.characters.last.contains(numRegex)) {
      return false;
    }
    if(!ta.characters.first.contains(numRegex)) {
      return false;
    }
    return true;
  }

  @override
  int? characterLimit() => 6;

  static double getDouble(String e) => double.parse(e.replaceAll(",", "."));
}

class HourInput extends KielTextInput {
  final int? hourLimit;

  const HourInput({this.hourLimit});

  @override
  List<TextInputFormatter> getFormatters() {
    return [
      HourValidator().getFormatter()
    ];
  }

  @override
  bool allowText(String s) => HourValidator().allowText(s);

  @override
  bool isValid(String t) {
    final h = int.tryParse(t);
    final limit = hourLimit ?? 24;
    if(h != null) {
      if(h > limit) {
        return false;
      }
    } else {
      return false;
    }
    return true;
  }

  @override
  int? characterLimit() => 2;

  static int getInt(String e) => int.parse(e);
}


