import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

abstract class KielTextValidator {
  bool validate(String t);

  TextInputFormatter getFormatter() {
    return TextInputFormatter.withFunction((oldValue, newValue) {
      if (validate(newValue.text)) {
        return newValue;
      } else {
        return oldValue;
      }
    });
  }
}

class IPAddressValidator extends KielTextValidator {
  final _numRegex = RegExp('[0-9]');

  @override
  bool validate(String t) {
    for (final c in t.characters) {
      if (!(c.contains(_numRegex) || c == '.')) {
        return false;
      }
    }
    return true;
  }
}

class DecimalValidator extends KielTextValidator {
  final _numRegex = RegExp('[0-9]');

  @override
  bool validate(String t) {
    for (final c in t.characters) {
      if (!(c.contains(_numRegex) || c == '.' || c == ',')) {
        return false;
      }
    }
    return true;
  }
}

TextEditingValue _formatDecimal(
    TextEditingValue oldValue, TextEditingValue newValue) {
  return TextEditingValue(
      text: newValue.text.replaceAll(".", ","),
      selection: newValue.selection,
      composing: newValue.composing);
}

TextInputFormatter getDecimalFormatter() {
  return TextInputFormatter.withFunction(_formatDecimal);
}

class IntegerValidator extends KielTextValidator {
  final _numRegex = RegExp('[0-9]');

  @override
  bool validate(String t) {
    for (final c in t.characters) {
      if (!(c.contains(_numRegex))) {
        return false;
      }
    }
    return true;
  }
}