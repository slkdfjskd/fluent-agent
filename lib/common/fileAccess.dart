import 'dart:io';

import 'package:fluent/api_bridge.dart';
import 'package:fluent/common/global.dart';
import 'package:fluent/common/log.dart';
import 'package:macos_secure_bookmarks/macos_secure_bookmarks.dart';

const fileKeyPrefix = "user_select_file_";

typedef StopFileAccess = void Function();

class StartFileAccessResult {
  final List<String> protoFiles;
  final List<String> importFiles;
  final StopFileAccess stopFileAccess;

  StartFileAccessResult(this.protoFiles, this.importFiles, this.stopFileAccess);
}

Future<void> saveBookmarks(
    int projectId, List<String> protoFiles, List<String> importFiles) async {
  if (Platform.isMacOS) {
    var secureBookmarks = SecureBookmarks();
    var files = [];
    files.addAll(protoFiles);
    files.addAll(importFiles);
    for (var file in files) {
      var bookmark = await secureBookmarks.bookmark(File(file));
      var result = await Global.rsLib
          .putConfig(key: _makeBookmarksKey(projectId, file), value: bookmark);
      if (result.code != Code.OK) {
        Log.e("saveBookmarks Global.rsLib.putConfig error:" + result.msg);
      }
    }
  }
}

String _makeBookmarksKey(int projectId, String file) {
  return fileKeyPrefix + projectId.toString() + file;
}

Future<bool> startFileAccess(
    int projectId, List<String> protoFiles, List<String> importFiles) async {
  if (Platform.isMacOS) {
    var r = await _startFileAccess(projectId, protoFiles, false);
    if (!r) {
      return false;
    }
    return await _startFileAccess(projectId, importFiles, true);
  }

  return true;
}

Future<bool> _startFileAccess(
    int projectId, List<String> files, bool isDir) async {
  var secureBookmarks = SecureBookmarks();
  for (var file in files) {
    var result =
        await Global.rsLib.getConfig(key: _makeBookmarksKey(projectId, file));
    if (result.code != Code.OK) {
      Log.e("saveBookmarks Global.rsLib.getConfig error:" + result.msg);
      continue;
    }
    if (result.data != null) {
      var resolvedFile = await secureBookmarks.resolveBookmark(result.data!,
          isDirectory: isDir);
      var isAgree = await secureBookmarks
          .startAccessingSecurityScopedResource(resolvedFile);
      // await secureBookmarks.stopAccessingSecurityScopedResource(resolvedFile);
      if (!isAgree) {
        return false;
      }
    }
  }

  return true;
}
