import 'package:expandable/expandable.dart';

import '../../api_bridge.dart';
import 'navRequest.dart';
import 'package:fluent/common/config/configKey.dart';
import '../../common/config/config.dart' as config;

class NavServiceVO {
  final String name;
  final List<NavRequestVO> requests = [];
  late final String expandStatusKey;
  ExpandableController expandableController = ExpandableController();

  NavServiceVO({
    required this.name,
    required List<NavRequestDTO> requestDTOs,
    required Map<String, String> configMap,
    required int projectId,
  }) {
    for (var req in requestDTOs) {
      requests.add(NavRequestVO(id: req.id, name: req.name));
    }
    expandStatusKey = navServiceExpandStatusKey(projectId, name);
    var expanded = configMap[expandStatusKey];
    expandableController.expanded =
        expanded == null ? false : expanded == 'true';
    expandableController.addListener(() {
      config.putBool(expandStatusKey, expandableController.expanded);
    });
  }

  NavServiceVO.update({
  required this.name,
  required List<NavRequestDTO> requestDTOs,
  required bool expanded,
    required int projectId,
  }) {
    for (var req in requestDTOs) {
      requests.add(NavRequestVO(id: req.id, name: req.name));
    }
    expandStatusKey = navServiceExpandStatusKey(projectId, name);
    expandableController.expanded = expanded;
    expandableController.addListener(() {
      config.putBool(expandStatusKey, expandableController.expanded);
    });
  }


  NavServiceVO.builder(
      {required this.name,
      required List<NavRequestVO> requests,
      required this.expandStatusKey,
      required this.expandableController}) {
    this.requests.addAll(requests);
  }


}
