import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:kielkanal/config_controller/config_controller.dart';
import 'package:kielkanal/config_screen/copy_request.dart';
import 'package:kielkanal/config_screen/weekday_screen.dart';
import 'package:kielkanal/main_screen.dart';
import 'package:provider/provider.dart';
import 'package:toml/toml.dart';

class ConfigScreen extends StatefulWidget {
  const ConfigScreen({Key? key}) : super(key: key);

  static const days = <String>[
    "Esmaspäev",
    "Teisipäev",
    "Kolmapäev",
    "Neljapäev",
    "Reede",
    "Laupäev",
    "Pühapäev"
  ];

  @override
  State<ConfigScreen> createState() => _ConfigScreenState();
}

class _ConfigScreenState extends State<ConfigScreen> {
  bool selectorOpen = true;
  int selectedDay = 0;

  void openDay(int i) {
    setState(() {
      selectorOpen = false;
      selectedDay = i;
    });
  }

  void closeDay() {
    setState(() => selectorOpen = true);
  }

  Widget daySelector() {
    final cardYellow = Colors.amber[300] ?? Colors.amber;

    return SizedBox(
      height: MediaQuery.of(context).size.height,
      width: MediaQuery.of(context).size.width,
      child: ColoredBox(
        color: cardYellow,
        child: ListView.builder(
            itemCount: ConfigScreen.days.length,
            itemBuilder: (BuildContext context, int i) {
              final day = ConfigScreen.days[i];
              return Consumer<ConfigController>(
                builder: (context, controller, child) {
                  final dayIsValid = controller.day(i).isValid();

                  return Card(
                      color: dayIsValid ? null : Colors.redAccent,
                      child: InkWell(
                          splashColor: cardYellow,
                          onTap: () {
                            FocusScope.of(context).unfocus();
                            openDay(i);
                          },
                          child: Padding(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              day.toUpperCase(),
                              style: GoogleFonts.secularOne(fontSize: 32),
                            ),
                          )));
                },
              );
            }),
      ),
    );
  }

  Widget dayLine(String name, Function() callback, {double fontSize = 24}) {
    return InkWell(
      onTap: callback,
      child: SizedBox(
        width: double.infinity,
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Center(
              child: Text(name,
                  style: GoogleFonts.secularOne(fontSize: fontSize))),
        ),
      ),
    );
  }

  Widget dayForm(BuildContext context, int dayNumber) {
    return Column(
      children: [
        Row(
          children: [
            InkWell(
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: IconButton(
                  icon: const Icon(Icons.keyboard_backspace),
                  onPressed: () {
                    FocusScope.of(context).unfocus();
                    closeDay();
                  },
                ),
              ),
            ),
            Center(
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: Text(
                  ConfigScreen.days[selectedDay].toUpperCase(),
                  style: GoogleFonts.secularOne(fontSize: 24),
                ),
              ),
            ),
            Expanded(
                child: Align(
              alignment: Alignment.centerRight,
              child: Consumer<ConfigController>(
                builder: (context, controller, child) {
                  return IconButton(
                    icon: const Icon(Icons.copy),
                    onPressed: () async {
                      final copyResult = await showDialog<CopyRequest>(
                          context: context,
                          builder: (context) {
                            final dayLinks = [];
                            for (int i = 0; i < ConfigScreen.days.length; i++) {
                              dayLinks.add(dayLine(
                                  ConfigScreen.days[i],
                                  () =>
                                      Navigator.pop(context, CopyRequest(i))));
                            }

                            return Dialog(
                              child: SingleChildScrollView(
                                child: Column(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    const Padding(
                                      padding: EdgeInsets.all(16.0),
                                      child: Text("Kopeeri sätted:",
                                          style: TextStyle(fontSize: 18)),
                                    ),
                                    dayLine(
                                        "Kõik päevad",
                                        () => Navigator.pop(
                                            context, CopyRequest(-3))),
                                    dayLine(
                                        "Kõik argipäevad",
                                        () => Navigator.pop(
                                            context, CopyRequest(-2))),
                                    const Divider(),
                                    ...dayLinks
                                  ],
                                ),
                              ),
                            );
                          });
                      if (copyResult != null) {
                        if (copyResult.day >= 0 && copyResult.day < 7) {
                          controller.copyDayOver(selectedDay, copyResult.day);
                        } else if (copyResult.day == -2) {
                          for (int i = 0; i < 5; i++) {
                            if (i != selectedDay) {
                              controller.copyDayOver(selectedDay, i);
                            }
                          }
                        } else if (copyResult.day == -3) {
                          for (int i = 0; i < 7; i++) {
                            if (i != selectedDay) {
                              controller.copyDayOver(selectedDay, i);
                            }
                          }
                        }
                      }
                    },
                  );
                },
              ),
            ))
          ],
        ),
        Expanded(child: WeekdayScreen(selectedDay))
      ],
    );
  }

  void confirmButtonAction(ConfigController controller) {
    final map = controller.toMap();
    final toml = TomlDocument.fromMap(map);
    debugPrint("$toml");
    ReloadNotification("Kinnitan muudatused", toml).dispatch(context);
  }

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;

    return Column(
      children: [
        Expanded(
          child: Stack(
            children: [
              IgnorePointer(
                ignoring: selectorOpen,
                child: dayForm(context, selectedDay),
              ),
              AnimatedPositioned(
                  duration: const Duration(milliseconds: 500),
                  left: selectorOpen ? 0.0 : -screenWidth,
                  curve: Curves.fastOutSlowIn,
                  child: daySelector()),
            ],
          ),
        ),
        Consumer<ConfigController>(
          builder: (context, controller, child) {
            final valid = controller.isValid();

            return Padding(
              padding: const EdgeInsets.all(8.0),
              child: SizedBox(
                  height: 64,
                  width: double.infinity,
                  child: ElevatedButton(
                      onPressed:
                          valid ? () => confirmButtonAction(controller) : null,
                      child: const Text("Kinnita muudatused"))),
            );
          },
        ),
      ],
    );
  }
}
