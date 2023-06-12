import 'package:flutter/cupertino.dart';

import '../../api_bridge.dart';
import '../../common/config/config.dart'as config;
import '../../common/config/configKey.dart' as configKey;
import '../../common/global.dart';

class Initializer with ChangeNotifier {

  InitData _initData = InitData([], {});

  InitData get initData => _initData;

  Future<void> init() async {
    var result = await Global.rsLib.listNavProject();
    if (result.code != Code.OK) {
      // TODO 错误消息
      return;
    }
    var configKeys = <String>[];
    for (var project in result.data) {
      configKeys.add(configKey.navProjectExpandStatusKey(project.projectId));
      for (var service in project.services) {
        configKeys.add(configKey.navServiceExpandStatusKey(project.projectId, service.name));
      }
    }
    // 首页需要知道log 窗口是否默认开启
    configKeys.add(configKey.logExpandStatus);
    configKeys.add(configKey.requestSelectedStatus);
    configKeys.add(configKey.requestTabSelectedStatus);
    configKeys.add(configKey.responseTabSelectedStatus);
    var configMap = await config.getInitConfig(configKeys);
    _initData = InitData(result.data, configMap);
  }
}

class InitData {
  final List<NavProjectDTO> _navProjectList;
  final Map<String, String> _configMap;

  InitData(this._navProjectList, this._configMap);

  List<NavProjectDTO> get navProjectList => _navProjectList;

  Map<String, String> get configMap => _configMap;
}
