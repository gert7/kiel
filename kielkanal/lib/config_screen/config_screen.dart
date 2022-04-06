import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:kielkanal/config_screen/day_screen.dart';

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
              return Card(
                  child: InkWell(
                      splashColor: cardYellow,
                      onTap: () => openDay(i),
                      child: Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: Text(
                          day.toUpperCase(),
                          style: GoogleFonts.secularOne(fontSize: 32),
                        ),
                      )));
            }),
      ),
    );
  }

  Widget dayForm(int i) {
    return Column(
      children: [
        Row(
          children: [
            InkWell(
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: IconButton(
                  icon: const Icon(Icons.keyboard_backspace),
                  onPressed: () => closeDay(),
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
              child: IconButton(
                icon: const Icon(Icons.copy),
                onPressed: () {},
              ),
            ))
          ],
        ),
        Expanded(child: DayScreen(selectedDay))
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;

    return Stack(
      children: [
        IgnorePointer(
          ignoring: selectorOpen,
          child: dayForm(selectedDay),
        ),
        AnimatedPositioned(
            duration: const Duration(milliseconds: 500),
            left: selectorOpen ? 0.0 : -screenWidth,
            curve: Curves.fastOutSlowIn,
            child: daySelector()),
      ],
    );
  }
}
