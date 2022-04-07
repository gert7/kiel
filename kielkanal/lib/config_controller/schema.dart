import 'package:flutter/services.dart';
import 'package:kielkanal/formatters.dart';

abstract class SchemaInput {}

class SchemaItem {
  final String tomlName;

  final SchemaInput input;

  SchemaItem(this.tomlName, this.input);
}

abstract class KielTextInput extends SchemaInput {
  List<TextInputFormatter> getFormatters();

  bool isValid(String s);
}

class DecimalInput extends KielTextInput {
  @override
  List<TextInputFormatter> getFormatters() {
    return [
      DecimalValidator().getFormatter(),
      getDecimalFormatter()
    ];
  }

  @override
  bool isValid(String s) => DecimalValidator().validate(s);
}

class IntegerInput extends KielTextInput {
  @override
  List<TextInputFormatter> getFormatters() {
    return [
      IntegerValidator().getFormatter()
    ];
  }

  @override
  bool isValid(String s) => IntegerValidator().validate(s);
}


