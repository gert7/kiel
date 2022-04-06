import 'package:flutter/material.dart';
import 'package:kielkanal/main_screen.dart';

const defaultIP = "127.0.0.1";

void main() {
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
  final TextEditingController _ip_controller = TextEditingController();

  @override
  void initState() {
    super.initState();
    _ip_controller.text = defaultIP;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // appBar: AppBar(
      //   title: Text(widget.title),
      // ),
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
                  style: Theme.of(context).textTheme.headline4,
                  textAlign: TextAlign.center,
                  controller: _ip_controller,
                  maxLength: 15,
                ),
              ),
              ElevatedButton(
                  onPressed: () {
                    Navigator.push(
                        context,
                        MaterialPageRoute(
                            builder: (BuildContext context) => WillPopScope(
                                onWillPop: () async => false,
                                child: const MainScreen())));
                  },
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Text(
                      "Sisene",
                      style: Theme.of(context).textTheme.headline5,
                    ),
                  ))
            ],
          ),
        )
      ]),
    );
  }
}
