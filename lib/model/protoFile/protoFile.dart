class ProtoFile {
  final List<String> _protoFiles = [];
  final List<String> _importPaths = [];

  List<String> get protoFiles => _protoFiles;
  List<String> get importPaths => _importPaths;

  setProtoFiles(List<String> files) {
    _protoFiles.addAll(files);
  }

  setImportPaths(List<String> paths) {
    _importPaths.addAll(paths);
  }

  addProtoFile(String file) {
    if (!_protoFiles.contains(file)) {
      _protoFiles.add(file);
    }
  }

  addImportPath(String path) {
    if (!_importPaths.contains(path)) {
      _importPaths.add(path);
    }
  }

  removeProtoFile(String file) {
    _protoFiles.remove(file);
  }

  removeImportPath(String path) {
    _importPaths.remove(path);
  }

  String getImportPath(int index) {
    return _importPaths[index];
  }

  String getProtoFile(int index) {
    return _protoFiles[index];
  }

  int importPathsLength() {
    return _importPaths.length;
  }

  int protoFilesLength() {
    return _protoFiles.length;
  }

  clear() {
    _protoFiles.clear();
    _importPaths.clear();
  }
}
