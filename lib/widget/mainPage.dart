import 'package:fluent/widget/bottomBar/bottomBar.dart';
import 'package:fluent/widget/bottomBar/logWidget.dart';
import 'package:fluent/widget/content.dart';
import 'package:flutter/material.dart';

import '../common/global.dart';
import '../common/log.dart';
import 'titleBar/titleBar.dart';

class MainPage extends StatefulWidget {
  const MainPage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  @override
  void initState() {
    Log.d('init state');
    super.initState();
  }

  @override
  void didChangeDependencies() {
    Log.d('did change dependencies');
    super.didChangeDependencies();
  }

  @override
  void didUpdateWidget(MainPage oldWidget) {
    Log.d('did update widget');
    super.didUpdateWidget(oldWidget);
  }

  @override
  void deactivate() {
    Log.d('deactivate');
    super.deactivate();
  }

  @override
  void dispose() {
    Log.d('dispose');
    super.dispose();
  }

  @override
  void reassemble() {
    Log.d('reassemble');
    super.reassemble();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Column(
      children: [
        titleBar(),
        const Expanded(child: ContentWidget()),
        const LogWidget(),
        const BottomBarWidget(),
      ],
    ));
    // return content();
  }
}
