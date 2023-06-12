import '../../api_bridge.dart';
import '../../common/global.dart';
import 'envVariable.dart';
import 'environment.dart';

class EnvironmentList {
  late List<EnvironmentVO> groupList = [];

  load() async {
    var result = await Global.rsLib.listEnv();

    if (result.code != Code.OK) {
      // TODO 错误处理
    }

    var groupList = result.data!;
    List<EnvironmentVO> newGroupList = [];
    for (var group in groupList) {
      List<EnvVariableVO> list = [];
      for (var env in group.list) {
        list.add(EnvVariableVO(env));
      }
      newGroupList.add(EnvironmentVO(envName: group.envName, list: list));
    }
    if (newGroupList.isNotEmpty) {
      this.groupList = newGroupList;
    } else {
      this.groupList = [EnvironmentVO(envName: "Default", list: [])];
    }
  }
}
