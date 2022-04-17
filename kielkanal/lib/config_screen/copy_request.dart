/// A copy request that is either the
/// index of a day of the week between 0-6, -2 representing
/// all weekdays (Mon-Fri), or -3 representing all days.
///
class CopyRequest {
  final int day;

  CopyRequest(this.day);
}
