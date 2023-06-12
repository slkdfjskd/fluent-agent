import 'package:fluent/api_bridge.dart';
import 'package:fluent/common/fileAccess.dart';
import 'package:fluent/model/protoFile/protoFile.dart';
import 'package:flutter/material.dart';
import 'package:fluent/common/log.dart';

import '../../common/global.dart';
import '../navProject/viewModel.dart';

class ProtoFileViewModel with ChangeNotifier {
  late NavProjectViewModel _navProjectViewModel;

  final Map<int, ProtoFile> _protoFileMap = {};

  void setNavProjectViewModel(NavProjectViewModel navProjectViewModel) {
    _navProjectViewModel = navProjectViewModel;
  }

  int protoFilesLength(int projectIndex) {
    var protoFile = _getProtoFileDTO(projectIndex);
    if (protoFile == null) {
      return 0;
    }
    return protoFile.protoFilesLength();
  }

  int importPathsLength(int projectIndex) {
    var protoFile = _getProtoFileDTO(projectIndex);
    if (protoFile == null) {
      return 0;
    }
    return protoFile.importPathsLength();
  }

  String protoFile(int projectIndex, int index) {
    var protoFileDTO = _getProtoFileDTO(projectIndex);
    if (protoFileDTO == null) {
      return '';
    }
    return protoFileDTO.getProtoFile(index);
  }

  String importPath(int projectIndex, int index) {
    var protoFileDTO = _getProtoFileDTO(projectIndex);
    if (protoFileDTO == null) {
      return '';
    }
    return protoFileDTO.getImportPath(index);
  }

  ProtoFile? _getProtoFileDTO(int projectIndex) {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    return _protoFileMap[navProject.projectId];
  }

  void addProtoFile(int projectIndex, String file) {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    loadProtoFileWithProjectId(navProject.projectId);
    var protoFile = _protoFileMap[navProject.projectId];
    if (protoFile != null) {
      protoFile.addProtoFile(file);
      notifyListeners();
    }
  }

  void addImportPath(int projectIndex, String path) {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    loadProtoFileWithProjectId(navProject.projectId);
    var protoFile = _protoFileMap[navProject.projectId];
    if (protoFile != null) {
      protoFile.addImportPath(path);
      notifyListeners();
    }
  }

  void removeProtoFile(int projectIndex, String file) {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    loadProtoFileWithProjectId(navProject.projectId);
    var protoFile = _protoFileMap[navProject.projectId];
    if (protoFile != null) {
      protoFile.removeProtoFile(file);
      notifyListeners();
    }
  }

  void removeImportPath(int projectIndex, String path) {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    loadProtoFileWithProjectId(navProject.projectId);
    var protoFile = _protoFileMap[navProject.projectId];
    if (protoFile != null) {
      protoFile.removeImportPath(path);
      notifyListeners();
    }
  }

  Future<void> loadProtoFile(int projectIndex) async {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    await loadProtoFileWithProjectId(navProject.projectId);
  }

  Future<void> loadProtoFileWithProjectId(int projectId) async {
    var protoFile = _protoFileMap[projectId];
    if (protoFile == null) {
      var result = await Global.rsLib.getProtoFile(projectId: projectId);
      if (result.code == Code.OK) {
        var protoFile = ProtoFile();
        protoFile.setProtoFiles(result.data!.protoFiles);
        protoFile.setImportPaths(result.data!.importPaths);
        _protoFileMap[projectId] = protoFile;
        notifyListeners();
      } else {
        // TODO 错误提示
      }
    }
  }

  Future<void> importProto(int projectIndex) async {
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    var protoFile = _protoFileMap[navProject.projectId];
    if (protoFile != null) {
      await saveBookmarks(
          navProject.projectId, protoFile.protoFiles, protoFile.importPaths);
      await startFileAccess(
          navProject.projectId, protoFile.protoFiles, protoFile.importPaths);
      var result = await Global.rsLib.importProto(
          projectId: navProject.projectId,
          protoFiles: protoFile.protoFiles,
          importPaths: protoFile.importPaths);
      if (result.code == Code.OK) {
        await _navProjectViewModel.updateNavProject(projectIndex, result.data!);
      } else {
        Log.e(result.msg);
        // TODO 错误提示
      }
    }
  }

  Future<void> refreshProto(int projectIndex) async {
    await loadProtoFile(projectIndex);
    var navProject = _navProjectViewModel.getNavProject(projectIndex);
    var protoFile = _protoFileMap[navProject.projectId];
    if (protoFile != null) {
      await startFileAccess(
          navProject.projectId, protoFile.protoFiles, protoFile.importPaths);
      var result = await Global.rsLib.importProto(
          projectId: navProject.projectId,
          protoFiles: protoFile.protoFiles,
          importPaths: protoFile.importPaths);
      if (result.code == Code.OK) {
        await _navProjectViewModel.updateNavProject(projectIndex, result.data!);
      } else {
        Log.e(result.msg);
      }
    }
  }

}
