import 'package:fluent/common/global.dart';
import 'package:fluent/widget/mainApp.dart';
import 'package:flutter/material.dart' as material;
import 'package:bitsdojo_window/bitsdojo_window.dart';

Future<void> main() async {
  await initRsLib();
  material.runApp(const MainApp());
  doWhenWindowReady(() {
    final win = appWindow;
    const initSize = material.Size(1150, 700);
    const minSize = material.Size(1100, 400);
    win.size = initSize;
    win.alignment = material.Alignment.topCenter;
    win.title = 'Fluent Agent';
    win.minSize = minSize;
    win.show();
  });
}
