import 'dart:ffi';

import '../api_bridge.dart';
import 'log.dart';

class Global {
  static FluentLib rsLib = loadRsLib();
  static ResultDTO? rsLibInitResult;
}

FluentLib loadRsLib() {
  var path = './fluent_lib.dylib';
  var dylib = DynamicLibrary.open(path);
  return FluentLibImpl(dylib);
}

Future<void> initRsLib() async {
  var result = await Global.rsLib.init();
  Global.rsLibInitResult = result;
  Log.i('init rs lib init result: $result');
}
