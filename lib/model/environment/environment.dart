import '../../api_bridge.dart';
import 'envVariable.dart';

class EnvironmentVO {
  String envName;
  bool envNameEdit;
  final List<EnvVariableVO> list;
  final varMap = <String, String>{};

  EnvironmentVO({
    required this.envName,
    required this.list,
    this.envNameEdit = false,
  }) {
    for (var v in list) {
      varMap[v.dto.name] = v.dto.value;
    }
  }

  removeVar(int index) {
    var v = list.removeAt(index);
    varMap.remove(v.dto.name);
  }

  addVar(EnvVariableVO vo) {
    list.add(vo);
    varMap[vo.dto.name] = vo.dto.value;
  }

  updateVar(int index, EnvVariableDTO dto) {
    var variable = list[index];
    varMap.remove(variable.dto.name);
    variable.dto = dto;
    varMap[dto.name] = dto.value;
  }

  String? getVarValue(String key) {
    return varMap[key];
  }
}
