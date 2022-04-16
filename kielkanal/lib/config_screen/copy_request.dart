/// A copy request that is either the
/// index of a day of the week between 0-6, or -2 representing
/// all weekdays (Mon-Fri)
///
class CopyRequest {
  final int day;

  CopyRequest(this.day);
}
