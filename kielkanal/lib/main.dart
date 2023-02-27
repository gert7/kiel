import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:kielkanal/formatters.dart';
import 'package:kielkanal/main_screen.dart';

import 'package:timezone/data/latest.dart' as tz;

const defaultIP = "192.168.1.138";

class IPCheckResult {
  final String ip;
  final bool result;

  IPCheckResult(this.ip, this.result);

  static Future<IPCheckResult> ipIsValid(String ip,
      {BuildContext? context}) async {
    const testString = "world";

    final client = HttpClient();
    try {
      final request = await client.get(ip, 8196, "/service/$testString");
      final response = await request.close();
      final stringData = await response.transform(utf8.decoder).join();
      if (stringData == "Kiel says hello, $testString!") {
        debugPrint("kiel says hello");
        return IPCheckResult(ip, true);
      } else {
        debugPrint("connection but no hello");
        return IPCheckResult(ip, false);
      }
    } catch (e) {
      debugPrint("$e");
      if (context != null) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(
          content: Text(e.toString()),
        ));
      }

      return IPCheckResult(ip, false);
    }
  }
}

void main() {
  tz.initializeTimeZones();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final TextEditingController _ipController = TextEditingController();
  final _ipFormatter = IPAddressValidator();
  final _ipStreamController = StreamController<IPCheckResult>();

  @override
  void initState() {
    super.initState();
    _ipController.text = defaultIP;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(children: [
        SizedBox(
          width: double.infinity,
          height: double.infinity,
          child: ColoredBox(
            color: Colors.blue,
            child: Image.asset("assets/sky-wallpaper-backgrounds.jpg",
                height: double.infinity, fit: BoxFit.cover),
          ),
        ),
        Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              SizedBox(
                width: 350.0,
                child: TextField(
                  style: Theme.of(context).textTheme.headlineMedium,
                  textAlign: TextAlign.center,
                  controller: _ipController,
                  inputFormatters: [_ipFormatter.getFormatter()],
                  maxLength: IPAddressValidator.maxLength,
                  keyboardType: TextInputType.number,
                ),
              ),
              StreamBuilder(
                builder: (context, snapshot) {
                  final result = snapshot.data;
                  if (snapshot.hasData && result is IPCheckResult) {
                    if (result.result) {
                      Timer(const Duration(milliseconds: 100), () async {
                        Navigator.push(
                            context,
                            MaterialPageRoute(
                                builder: (BuildContext context) => WillPopScope(
                                    onWillPop: () async => false,
                                    child: MainScreenDBFilter(result.ip))));
                      });
                    }
                  }
                  return const SizedBox(
                    width: 2.0,
                    height: 2.0,
                  );
                },
                stream: _ipStreamController.stream,
              ),
              ElevatedButton(
                  onPressed: () async {
                    _ipStreamController.add(await IPCheckResult.ipIsValid(
                        _ipController.text,
                        context: context));
                  },
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Text(
                      "Sisene",
                      style: Theme.of(context).textTheme.headlineSmall,
                    ),
                  ))
            ],
          ),
        )
      ]),
    );
  }
}
