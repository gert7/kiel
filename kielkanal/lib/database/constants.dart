import 'package:timezone/timezone.dart' as tz;

tz.Location marketTimeZone() {
  return tz.getLocation("Europe/Berlin");
}

tz.TZDateTime marketTimeNow() {
  return tz.TZDateTime.now(marketTimeZone());
}

tz.Location localTimeZone() {
  return tz.getLocation("Europe/Tallinn");
}

tz.TZDateTime localTimeNow() {
  return tz.TZDateTime.now(marketTimeZone());
}