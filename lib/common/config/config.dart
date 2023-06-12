import 'package:fluent/api_bridge.dart';
import 'package:fluent/common/global.dart';

final configs = <String, String>{};

void put(String key, String value) {
  configs[key] = value;
  Future(() async {
    var result = await Global.rsLib.putConfig(key: key, value: value);
    if (result.code != Code.OK) {
      // TODO
      return;
    }
  });
}

void delete(List<String> keys) {
  for (var key in keys) {
    configs.remove(key);
  }
  Future.delayed(const Duration(seconds: 0), () async {
    var result = await Global.rsLib.deleteConfigs(keys: keys);
    if (result.code != Code.OK) {
      // TODO
      return;
    }
  });
}

void putBool(String key, bool value) {
  put(key, value.toString());
}

void putInt(String key, int value) {
  put(key, value.toString());
}

Future<bool> getBool(String key, bool def) async {
  String value = await getString(key);
  if (value.isEmpty) {
    return def;
  }
  return "true" == value;
}

Future<int> getInt(String key, int def) async {
  String value = await getString(key);
  if (value.isEmpty) {
    return def;
  }
  var result = int.tryParse(value);
  if (result == null) {
    return def;
  }
  return result;
}

Future<String> getString(String key) async {
  var value = configs[key];
  if (value != null) {
    return value;
  }
  var result = await Global.rsLib.getConfig(key: key);
  if (result.code != Code.OK) {
    // TODO 错误显示
    return "";
  }
  if (result.data != null) {
    configs[key] = result.data!;
    return result.data!;
  }
  return "";
}

Future<Map<String, String>> getInitConfig(List<String> keys) async {
  var result = await Global.rsLib.getBatchConfig(keys: keys);
  if (result.code != Code.OK) {
    // TODO 错误显示
    return {};
  }
  var map = <String, String>{};
  for (var config in result.data) {
    configs[config.name] = config.value;
    map[config.name] = config.value;
  }
  return map;
}
