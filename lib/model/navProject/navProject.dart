import 'package:expandable/expandable.dart';

import 'package:fluent/common/config/configKey.dart' as configKey;
import '../../common/config/config.dart' as config;
import '../../api_bridge.dart';
import 'navRequest.dart';
import 'navService.dart';

class NavProjectVO {
  late final int id;
  final int projectId;
  late  String projectName;
  final ReqType reqType;
  final List<NavServiceVO> services = [];
  final List<NavRequestVO> requests = [];
  final int orderNo;
  late final String _expandStatusKey;
  bool editName = false;
  ExpandableController expandableController = ExpandableController();

  int? originIndex;

  NavProjectVO({
    required this.id,
    required this.projectId,
    required this.projectName,
    required this.reqType,
    required List<NavServiceDTO> serviceDTOs,
    required List<NavRequestDTO> requestDTOs,
    required this.orderNo,
    required this.editName,
    required Map<String, String> configMap,
  }) {
    for (var service in serviceDTOs) {
      services.add(NavServiceVO(
          name: service.name,
          requestDTOs: service.requests,
          configMap: configMap,
          projectId: projectId));
    }
    for (var req in requestDTOs) {
      requests.add(NavRequestVO(id: req.id, name: req.name));
    }
    expandableController = ExpandableController();
    _expandStatusKey = configKey.navProjectExpandStatusKey(projectId);
    var expanded = configMap[_expandStatusKey];
    expandableController.expanded =
        expanded == null ? false : expanded == 'true';
    expandableController.addListener(() {
      config.putBool(_expandStatusKey, expandableController.expanded);
    });
  }

  NavProjectVO.update({
    required this.id,
    required this.projectId,
    required this.projectName,
    required this.reqType,
    required List<NavServiceDTO> serviceDTOs,
    required List<NavRequestDTO> requestDTOs,
    required this.orderNo,
    required this.editName,
    required bool expanded,
  }) {
    for (var service in serviceDTOs) {
      services.add(NavServiceVO.update(
          name: service.name,
          requestDTOs: service.requests,
          expanded: expanded,
          projectId: projectId));
    }
    for (var req in requestDTOs) {
      requests.add(NavRequestVO(id: req.id, name: req.name));
    }
    expandableController.expanded = expanded;
    expandableController.addListener(() {
      config.putBool(_expandStatusKey, expandableController.expanded);
    });
  }

  NavProjectVO.builder({
    required this.id,
    required this.projectId,
    required this.projectName,
    required this.reqType,
    required List<NavServiceVO> serviceVOs,
    required List<NavRequestVO> requestVOs,
    required this.orderNo,
    required this.editName,
    required this.expandableController,
    required this.originIndex,
  }) {
    services.addAll(serviceVOs);
    requests.addAll(requestVOs);
  }

}
