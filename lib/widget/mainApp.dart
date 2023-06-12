import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../model/provider.dart';
import 'mainPage.dart';

class MainApp extends StatelessWidget {
  const MainApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: providers(),
      child: MaterialApp(
        title: 'Fluent Agent',
        theme: ThemeData(
          primarySwatch: Colors.blue,
        ),
        home: const MainPage(title: 'Fluent'),
      ),
    );
  }
}
