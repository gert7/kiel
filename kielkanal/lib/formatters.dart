import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

final numRegex = RegExp('[0-9]');

abstract class KielTextValidator {
  bool allowText(String t);

  TextInputFormatter getFormatter() {
    return TextInputFormatter.withFunction((oldValue, newValue) {
      if (allowText(newValue.text)) {
        return newValue;
      } else {
        return oldValue;
      }
    });
  }
}

class IPAddressValidator extends KielTextValidator {
  static const maxLength = 15;

  @override
  bool allowText(String t) {
    for (final c in t.characters) {
      if (!(c.contains(numRegex) || c == '.')) {
        return false;
      }
    }
    return true;
  }
}

class EMWhValidator extends KielTextValidator {
  @override
  bool allowText(String t) {
    for (final c in t.characters) {
      if (!(c.contains(numRegex) || c == '.' || c == ',')) {
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

class HourValidator extends KielTextValidator {
  @override
  bool allowText(String t) {
    for (final c in t.characters) {
      if (!(c.contains(numRegex))) {
        return false;
      }
    }
    return true;
  }
}
