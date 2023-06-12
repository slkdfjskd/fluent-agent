

import 'package:fluent/common/config/config.dart';
import 'package:fluent/common/config/configKey.dart';
import 'package:fluent/model/initData/initData.dart';
import 'package:flutter/material.dart';

class BottomBarViewModel with ChangeNotifier {
  bool _isOpenWithLog = false ;


  set initData(InitData value) {
    var status = value.configMap[logExpandStatus];
    _isOpenWithLog = status == null ? false : status == 'true';
    notifyListeners();
  }

  bool get isOpenWithLog => _isOpenWithLog;

  set isOpenWithLog(bool value) {
    _isOpenWithLog = value;
    putBool(logExpandStatus, value);
    notifyListeners();
  }
}