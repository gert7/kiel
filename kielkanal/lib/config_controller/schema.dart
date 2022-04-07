import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:kielkanal/formatters.dart';

abstract class SchemaInput {}

class SchemaItem {
  final String tomlName;

  final String prettyName;

  final SchemaInput input;

  SchemaItem(this.tomlName, this.input, this.prettyName);
}

abstract class KielTextInput extends SchemaInput {
  List<TextInputFormatter> getFormatters();

  bool allowText(String s);

  bool isValid(String t);

  int? characterLimit();
}

class EMWhInput extends KielTextInput {
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
    print("Allowtext");
    if(!allowText(ta)) { return false; }
    print("last");
    if(!ta.characters.last.contains(numRegex)) {
      return false;
    }
    print("first");
    if(!ta.characters.first.contains(numRegex)) {
      return false;
    }
    return true;
  }

  @override
  int? characterLimit() => 5;
}

class HourInput extends KielTextInput {
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
    if(h != null) {
      if(h > 24) {
        return false;
      }
    } else {
      return false;
    }
    return true;
  }

  @override
  int? characterLimit() => 2;
}


