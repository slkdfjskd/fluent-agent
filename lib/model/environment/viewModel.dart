import 'package:fluent/api_bridge.dart';
import 'package:fluent/common/global.dart';
import 'package:flutter/material.dart';

import 'envVariable.dart';
import 'environment.dart';

const defaultName = 'Def-';
const defaultVariableValue = 'Default Value';

// const variableNoSelected = -1;
const noSelected = -1;

class EnvironmentViewModel with ChangeNotifier {
  List<EnvironmentVO> _envList = [];
  Set<String> _envNameSet = {};

  bool _isOpen = true;
  int _envManagerSelectIndex = noSelected;
  int _variableSelectIndex = noSelected;
  int _envSelectIndex = noSelected;

  int get envSelectIndex => _envSelectIndex;

  List<EnvironmentVO> get envList => _envList;

  bool get isOpen => _isOpen;

  String envName() {
    if (envSelectIndex >= 0) {
      return _envList[envSelectIndex].envName;
    }
    return "";
  }

  String parseUrl(String url) {
    if (envSelectIndex < 0) {
      return url;
    }
    var env = envList[envSelectIndex];
    var varMap = <String, String>{};
    var regExpItem = RegExp("\{\{[\\w-]*\}\}");
    var matchVarItems = regExpItem.allMatches(url).toList();
    var regExpVarName = RegExp("[\\w-]*");
    for (var item in matchVarItems) {
      var matchItem = item.input.substring(item.start, item.end);
      if (matchItem.isEmpty) {
        continue;
      }
      var vars = regExpVarName.allMatches(matchItem).toList();
      for (var v in vars) {
        var matchVar = v.input.substring(v.start, v.end);
        if (matchVar.isEmpty) {
          continue;
        }
        var varValue = env.varMap[matchVar];
        if (varValue == null) {
          continue;
        }
        varMap[matchItem] = varValue;
      }
    }

    varMap.forEach((key, value) {
      url = url.replaceFirst(key, value);
    });

    return url;
  }

  bool isSelectedEnv() {
    return _envSelectIndex != noSelected;
  }

  revertOpen() {
    _isOpen = !_isOpen;
    notifyListeners();
  }

  setEnvironmentList(List<EnvironmentVO> list) {
    _envList = list;
    if (list.isNotEmpty) {
      _envSelectIndex = 0;
      _envManagerSelectIndex = 0;
    }
    for (var env in _envList) {
      _envNameSet.add(env.envName);
    }
    notifyListeners();
  }

  selectEnv(int? index) {
    if (index == null) {
      return;
    }
    _envSelectIndex = index;
    notifyListeners();
  }

  Future<String> getDefaultName() async {
    var result = await Global.rsLib.uniqueId();
    if (result.code != Code.OK) {
      // TODO 提示错误
      return "";
    }
    var id = result.data;
    return defaultName + id.toString();
  }

  createEnv() async {
    var name = await getDefaultName();
    _envNameSet.add(name);
    _envList.add(EnvironmentVO(envName: name, list: [], envNameEdit: true));
    _envManagerSelectIndex = _envList.length - 1;
    if (!isSelectedEnv()) {
      _envSelectIndex = 0;
    }
    notifyListeners();
  }

  editEnvName(int index, bool edit) {
    if (index >= _envList.length) {
      return;
    }
    _envList[index].envNameEdit = edit;
    notifyListeners();
  }

  updateEnvName(int index, String newName) async {
    if (index < 0) {
      return;
    }
    if (index >= _envList.length) {
      return;
    }
    var env = _envList[index];
    if (newName.isEmpty || env.envName == newName) {
      return;
    }

    // env 名字重复不修改
    if (_envNameSet.contains(newName)) {
      return;
    }

    var result = await Global.rsLib
        .updateEnvName(newName: newName, oldName: env.envName);
    if (result.code != Code.OK) {
      return;
    }

    _envNameSet.remove(env.envName);
    _envNameSet.add(newName);

    env.envName = newName;
    for (var variable in env.list) {
      var dto = variable.dto;
      variable.dto = EnvVariableDTO(
          id: dto.id, envName: newName, name: dto.name, value: dto.value);
    }
    notifyListeners();
  }

  createVariable() async {
    var name = await getDefaultName();
    var envGroup = currentManagerEnv();
    if (envGroup == null) {
      return;
    }
    var param = CreateEnvironment(
        name: name, value: defaultVariableValue, envName: envGroup.envName);
    var result = await Global.rsLib.createEnv(param: param);
    if (result.code != Code.OK) {
      return;
    }

    envGroup.addVar(EnvVariableVO(result.data!, nameEdit: true));
    notifyListeners();
  }

  updateVariable(int index, {String? name, String? value}) async {
    var envGroup = currentManagerEnv();
    if (envGroup == null) {
      return;
    }
    if (index >= envGroup.list.length) {
      return;
    }

    var variable = envGroup.list[index];
    if ((name == null || name == variable.dto.name) &&
        (value == null || value == variable.dto.value)) {
      return;
    }

    value = value ?? variable.dto.value;
    name = name == null || name.isEmpty ? variable.dto.name : name;
    var param =
        UpdateEnvironment(id: variable.dto.id, name: name!, value: value!);
    var result = await Global.rsLib.updateEnvVariable(param: param);
    if (result.code != Code.OK) {
      // TODO
    } else {
      envGroup.updateVar(
          index,
          EnvVariableDTO(
              id: variable.dto.id,
              envName: variable.dto.envName,
              name: name,
              value: value));
    }
    notifyListeners();
  }

  deleteCurrentSelectedEnv() async {
    var envGroup = currentManagerEnv();
    if (envGroup == null) {
      return;
    }

    var result = await Global.rsLib.deleteEnv(envName: envGroup.envName);
    if (result.code != Code.OK) {
      // TODO
      return;
    }
    _envList.remove(envGroup);

    if (_envList.isEmpty) {
      _envManagerSelectIndex = noSelected;
      _envSelectIndex = noSelected;
    } else {
      if (_envManagerSelectIndex > 0) {
        _envManagerSelectIndex -= 1;
      }

      if (_envSelectIndex > 0) {
        _envSelectIndex -= 1;
      }
    }
    notifyListeners();
  }

  deleteCurrentSelectedVariable() async {
    var index = _variableSelectIndex;
    if (index < 0) {
      return;
    }
    var envGroup = currentManagerEnv();
    if (envGroup == null) {
      return;
    }
    if (index >= envGroup.list.length) {
      return;
    }
    var variable = envGroup.list[index];
    var result = await Global.rsLib.deleteEnvVariable(id: variable.dto.id);
    if (result.code != Code.OK) {
      return;
    }
    envGroup.removeVar(index);
    _variableSelectIndex = noSelected;
    notifyListeners();
  }

  selectManagerEnv(int index) {
    _envManagerSelectIndex = index;
    _variableSelectIndex = noSelected;
    notifyListeners();
  }

  selectVariable(int index) {
    _variableSelectIndex = index;
    notifyListeners();
  }

  EnvironmentVO? currentManagerEnv() {
    if (_envList.isEmpty) {
      return null;
    }
    if (_envManagerSelectIndex < 0) {
      return null;
    }
    if (_envManagerSelectIndex >= _envList.length) {
      return null;
    }
    return _envList[_envManagerSelectIndex];
  }

  EnvVariableVO? currentVariable() {
    var index = _variableSelectIndex;
    if (index < 0) {
      return null;
    }
    var envGroup = currentManagerEnv();
    if (envGroup == null) {
      return null;
    }
    if (index >= envGroup.list.length) {
      return null;
    }
    return envGroup.list[index];
  }

  int get envManagerSelectIndex => _envManagerSelectIndex;

  int get variableSelectIndex => _variableSelectIndex;

  editName(int index, bool edit) {
    currentManagerEnv()?.list[index].nameEdit = edit;
    notifyListeners();
  }

  editValue(int index, bool edit) {
    currentManagerEnv()?.list[index].valueEdit = edit;
    notifyListeners();
  }
}
