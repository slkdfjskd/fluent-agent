import 'package:expandable/expandable.dart';
import 'package:fluent/api_bridge.dart';
import 'package:fluent/common/config/config.dart';
import 'package:fluent/common/config/configKey.dart' as configKey;
import 'package:fluent/common/log.dart';
import 'package:fluent/model/navProject/navService.dart';
import 'package:fluent/model/request/viewModel.dart';
import 'package:flutter/material.dart';

import '../../common/config/configKey.dart';
import '../../common/global.dart';
import 'navProject.dart';
import '../initData/initData.dart';
import 'navRequest.dart';

const noSelect = -1;

class NavProjectViewModel with ChangeNotifier {
  final RequestViewModel _requestViewModel;
  List<NavProjectVO> _navProjectList = [];
  List<NavProjectVO> _originNavProjectList = [];

  int _selectedRequestId = 0;

  final TextEditingController _filterController = TextEditingController();

  NavProjectViewModel(this._requestViewModel) {
    _filterController.addListener(() {
      Log.d("_filterController ${_filterController.text}");
      filter(_filterController.text);
      notifyListeners();
    });
  }

  TextEditingController get filterController => _filterController;

  RequestViewModel get requestViewModel => _requestViewModel;

  bool isSelectedRequest(int requestId) {
    return _selectedRequestId == requestId;
  }

  void setSelectRequestId(int requestId) {
    _selectedRequestId = requestId;
    putInt(requestSelectedStatus, requestId);
    notifyListeners();
  }

  int length() {
    return _navProjectList.length;
  }

  NavProjectVO getNavProject(int index) {
    return _navProjectList[index];
  }

  void setEditProjectNameState(int index, bool edit) {
    var navProject = _navProjectList[index];
    navProject.editName = edit;
    notifyListeners();
  }

  bool isEditProjectNameState(int index) {
    var navProject = _navProjectList[index];
    return navProject.editName;
  }

  void initData(InitData data) {
    List<NavProjectVO> newList = [];

    for (var dto in data.navProjectList) {
      newList.add(NavProjectVO(
          id: dto.id,
          projectId: dto.projectId,
          projectName: dto.projectName,
          reqType: dto.reqType,
          serviceDTOs: dto.services,
          requestDTOs: dto.requests,
          orderNo: dto.orderNo,
          editName: false,
          configMap: data.configMap));
    }
    _originNavProjectList = newList;
    filter("");
    var value = data.configMap[configKey.requestSelectedStatus] ?? "0";
    var status = int.tryParse(value);
    _selectedRequestId = status ?? 0;
    _initSelectedRequest(_selectedRequestId);
    notifyListeners();
  }

  void _initSelectedRequest(int requestId) {
    Future(() async {
      if (requestId != 0) {
        await requestViewModel.show(requestId);
      }
    });
  }

  void addNavProject(NavProjectDTO navProject) {
    _originNavProjectList.insert(
        0,
        NavProjectVO(
            id: navProject.id,
            projectId: navProject.projectId,
            projectName: navProject.projectName,
            reqType: navProject.reqType,
            serviceDTOs: navProject.services,
            requestDTOs: navProject.requests,
            orderNo: navProject.orderNo,
            editName: true,
            configMap: <String, String>{}));
    _filterController.text = "";
    filter(_filterController.text);
    notifyListeners();
  }

  Future<void> updateNavProject(int index, NavProjectDTO navProjectDTO) async {
    var project = _navProjectList[index];
    if (project.originIndex != null) {
      index = project.originIndex!;
    }

    _originNavProjectList[index] = NavProjectVO.update(
        id: navProjectDTO.id,
        projectId: navProjectDTO.projectId,
        projectName: navProjectDTO.projectName,
        reqType: navProjectDTO.reqType,
        serviceDTOs: navProjectDTO.services,
        requestDTOs: navProjectDTO.requests,
        orderNo: navProjectDTO.orderNo,
        editName: false,
        expanded: true);

    var requestIds = <int>[];
    for (var service in navProjectDTO.services) {
      for (var request in service.requests) {
        requestIds.add(request.id);
      }
    }
    for (var request in navProjectDTO.requests) {
      requestIds.add(request.id);
    }
    _requestViewModel.deleteCache(requestIds);

    _filterController.text = "";
    filter(_filterController.text);
    notifyListeners();
  }

  Future<void> updateProjectName(int index, String newName) async {
    var project = _navProjectList[index];
    var result = await Global.rsLib
        .updateProjectName(projectId: project.projectId, newName: newName);
    if (result.code == Code.OK) {
      project.projectName = newName;
      if (project.originIndex != null) {
        index = project.originIndex!;
      }
      _originNavProjectList[index].projectName = newName;
    } else {
      // TODO 显示错误信息
    }
    notifyListeners();
  }

  Future<void> deleteNavProject(int index) async {
    var project = _navProjectList[index];
    var result = await Global.rsLib.deleteProject(projectId: project.projectId);
    if (result.code == Code.OK) {
      _navProjectList.removeAt(index);
      if (project.originIndex != null) {
        index = project.originIndex!;
      }
      _originNavProjectList.removeAt(index);
    } else {
      // TODO 显示错误信息
    }
    notifyListeners();
  }

  Future<void> createProject(
      {required String name, required ReqType reqType}) async {
    var result = await Global.rsLib.createProject(name: name, reqType: reqType);
    if (result.code == Code.OK) {
      addNavProject(result.data!);
    } else {
      // TODO 显示错误信息
    }
  }

  void filter(String text) {
    _navProjectList = [];
    if (text.isEmpty) {
      _navProjectList.addAll(_originNavProjectList);
      return;
    }

    _originNavProjectList.asMap().forEach((index, project) {
      var newServices = <NavServiceVO>[];
      for (var service in project.services) {
        var newRequests = <NavRequestVO>[];
        for (var request in service.requests) {
          if (request.name.toLowerCase().contains(text.toLowerCase())) {
            newRequests.add(request);
          }
        }
        if (newRequests.isNotEmpty) {
          ExpandableController controller =
          ExpandableController(initialExpanded: true);
          newServices.add(NavServiceVO.builder(
              name: service.name,
              requests: newRequests,
              expandStatusKey: service.expandStatusKey,
              expandableController: controller));
        }
      }
      if (newServices.isNotEmpty) {
        project.expandableController.expanded = true;
        ExpandableController controller =
        ExpandableController(initialExpanded: true);
        _navProjectList.add(NavProjectVO.builder(
            id: project.id,
            projectId: project.projectId,
            projectName: project.projectName,
            reqType: project.reqType,
            serviceVOs: newServices,
            requestVOs: project.requests,
            orderNo: project.orderNo,
            editName: project.editName,
            expandableController: controller,
            originIndex: index));
      }
    });
  }
}
