import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/navProject/viewModel.dart';
import '../../model/protoFile/viewModel.dart';

enum _itemValue {
  rename,
  importProto,
  refreshProto,
  editProto,
  delete,
}

class _Item {
  String name;
  _itemValue value;

  _Item(this.name, this.value);
}

var _projectPopupItems = {
  _Item('Rename', _itemValue.rename),
  _Item('Import Proto', _itemValue.importProto),
  _Item('Refresh Proto', _itemValue.refreshProto),
  _Item('Delete', _itemValue.delete),
};

void showPopupMenu(
    BuildContext ctx,
    double dx,
    double dy,
    NavProjectViewModel navProjectViewModel,
    ProtoFileViewModel protoFileViewModel,
    int projectIndex) {
  List<PopupMenuItem> items = [];
  for (var item in _projectPopupItems) {
    items.add(PopupMenuItem(
      child: Text(item.name, style: const TextStyle(fontSize: fontSize)),
      value: item.value,
      height: 25,
    ));
  }
  var r = showMenu(
      context: ctx,
      position: RelativeRect.fromLTRB(dx, dy, dx, dy),
      color: popupMenuColor,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(3),
      ),
      items: items);
  r.then((value) {
    _selected(
        ctx, navProjectViewModel, protoFileViewModel, projectIndex, value);
  });
}

Future<void> _selected(
    BuildContext ctx,
    NavProjectViewModel navProjectViewModel,
    ProtoFileViewModel protoFileViewModel,
    int projectIndex,
    _itemValue value) async {
  // await protoFileViewModel.loadProtoFile(projectIndex);
  Log.d('select:$value');
  switch (value) {
    case _itemValue.rename:
      navProjectViewModel.setEditProjectNameState(projectIndex, true);
      break;
    case _itemValue.importProto:
      showImportProtoDialog(ctx, protoFileViewModel, projectIndex);
      break;
    case _itemValue.refreshProto:
      protoFileViewModel.refreshProto(projectIndex);
      break;
    case _itemValue.editProto:
      break;
    case _itemValue.delete:
      bool? delete = await showDeleteConfirmDialog(ctx);
      Log.d('showDeleteConfirmDialog delete: $delete');
      if (delete != null && delete) {
        navProjectViewModel.deleteNavProject(projectIndex);
      }
      break;
  }
}

Future<void> showImportProtoDialog(BuildContext ctx,
    ProtoFileViewModel protoFileViewModel, int projectIndex) async {
  return showDialog(
      context: ctx,
      barrierDismissible: false,
      barrierColor: lightDialogBarrierColor,
      builder: (ctx) {
        Log.d('showImportProtoDialog build');
        var viewModel = Provider.of<ProtoFileViewModel>(ctx);
        viewModel.loadProtoFile(projectIndex);
        var title = 'Import proto from local';
        return AlertDialog(
          title: Text(title),
          titleTextStyle: const TextStyle(
              fontSize: fontSize + 2,
              color: Colors.black87,
              fontWeight: FontWeight.bold),
          content: _ImportProtoWidget(projectIndex, viewModel),
          contentTextStyle:
              const TextStyle(fontSize: fontSize, color: Colors.black87),
          actions: <Widget>[
            OutlinedButton(
              child: const Text("Cancel", style: TextStyle(fontSize: fontSize)),
              style: ButtonStyle(
                  padding: MaterialStateProperty.all(const EdgeInsets.only(
                      left: 40, right: 40, top: 7, bottom: 7))),
              onPressed: () {
                Navigator.of(ctx).pop();
              }, // 关闭对话框
            ),
            ElevatedButton(
              child:
                  const Text("Confirm", style: TextStyle(fontSize: fontSize)),
              style: ButtonStyle(
                  padding: MaterialStateProperty.all(const EdgeInsets.only(
                      left: 40, right: 40, top: 7, bottom: 7))),
              onPressed: () {
                viewModel.importProto(projectIndex);
                Navigator.of(ctx).pop(true);
              },
            ),
          ],
          // buttonPadding: EdgeInsets.all(20),
          actionsAlignment: MainAxisAlignment.center,
          actionsPadding: const EdgeInsets.only(bottom: 5),
        );
      });
}

Future<bool?> showDeleteConfirmDialog(BuildContext ctx) {
  return showDialog<bool>(
    barrierColor: darkDialogBarrierColor,
    context: ctx,
    builder: (context) {
      return AlertDialog(
        title: const Text('Delete one project?',
            style: TextStyle(fontWeight: FontWeight.bold)),
        titleTextStyle:
            const TextStyle(fontSize: fontSize + 2, color: Colors.black87),
        content: const Text("Delete the project Project1657100?"),
        contentTextStyle:
            const TextStyle(fontSize: fontSize, color: Colors.black87),
        actions: <Widget>[
          OutlinedButton(
            child: const Text("Cancel", style: TextStyle(fontSize: fontSize)),
            style: ButtonStyle(
                padding: MaterialStateProperty.all(const EdgeInsets.only(
                    left: 40, right: 40, top: 7, bottom: 7))),
            onPressed: () => Navigator.of(context).pop(), // 关闭对话框
          ),
          ElevatedButton(
            child: const Text("Delete", style: TextStyle(fontSize: fontSize)),
            style: ButtonStyle(
                padding: MaterialStateProperty.all(const EdgeInsets.only(
                    left: 40, right: 40, top: 7, bottom: 7))),
            onPressed: () {
              //关闭对话框并返回true
              Navigator.of(context).pop(true);
            },
          ),
        ],
        actionsAlignment: MainAxisAlignment.center,
        actionsPadding: const EdgeInsets.only(bottom: 5),
      );
    },
  );
}

class _ImportProtoWidget extends StatefulWidget {
  final int _projectIndex;

  final ProtoFileViewModel _viewModel;

  const _ImportProtoWidget(this._projectIndex, this._viewModel, {Key? key})
      : super(key: key);

  @override
  _ImportProtoState createState() => _ImportProtoState();
}

class _ImportProtoState extends State<_ImportProtoWidget> {
  @override
  Widget build(BuildContext context) {
    var projectIndex = widget._projectIndex;
    var viewModel = widget._viewModel;
    return Scrollbar(
        child: SingleChildScrollView(
            child: Container(
                width: 500,
                height: 600,
                constraints: const BoxConstraints(maxHeight: 500),
                child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Select local proto file',
                        textAlign: TextAlign.left,
                        style: TextStyle(fontWeight: FontWeight.bold),
                      ),
                      Container(
                          margin: const EdgeInsets.only(top: 10),
                          child: const Text(
                              'Select .proto file from your local system to use the services and methods.')),
                      Container(
                          margin: const EdgeInsets.only(top: 10, bottom: 5),
                          child: OutlinedButton.icon(
                              style: TextButton.styleFrom(
                                  foregroundColor: rippleColor),
                              onPressed: () {
                                Log.d("onPressed");
                                _pickProtoFile(
                                    projectIndex, viewModel);
                              },
                              icon: const Icon(
                                Icons.add,
                                color: blackFontColor,
                                size: iconSize,
                              ),
                              label: const Text('Add a proto file',
                                  style: TextStyle(
                                      fontSize: fontSize,
                                      color: blackFontColor)))),
                      ListView.builder(
                          padding: const EdgeInsets.only(bottom: 20),
                          shrinkWrap: true,
                          itemCount: viewModel.protoFilesLength(projectIndex),
                          itemBuilder: (ctx, index) => _item(
                              true,
                              viewModel,
                              projectIndex,
                              viewModel.protoFile(projectIndex, index))),
                      const Text(
                        'Import paths',
                        style: TextStyle(fontWeight: FontWeight.bold),
                      ),
                      Container(
                          margin: const EdgeInsets.only(top: 10),
                          child: const Text(
                              'Specify import paths to look for .proto files when resolving “import” directives.')),
                      Container(
                          margin: const EdgeInsets.only(top: 10, bottom: 5),
                          child: OutlinedButton.icon(
                              style: TextButton.styleFrom(
                                  foregroundColor: rippleColor),
                              onPressed: () {
                                Log.d("onPressed");
                                _pickImportPath(projectIndex, viewModel);
                              },
                              icon: const Icon(
                                Icons.add,
                                color: blackFontColor,
                                size: iconSize,
                              ),
                              label: const Text('Add an import path',
                                  style: TextStyle(
                                      fontSize: fontSize,
                                      color: blackFontColor)))),
                      ListView.builder(
                          shrinkWrap: true,
                          itemCount: viewModel.importPathsLength(projectIndex),
                          itemBuilder: (ctx, index) => _item(
                              false,
                              viewModel,
                              projectIndex,
                              viewModel.importPath(projectIndex, index))),
                    ]))));
  }

  Widget _item(bool isProto, ProtoFileViewModel protoFileViewModel,
      int projectIndex, String value) {
    return SizedBox(
        height: 30,
        child: Row(children: [
          SizedBox(
              width: 30,
              child: IconButton(
                  splashRadius: 15,
                  iconSize: iconSize,
                  onPressed: () {
                    if (isProto) {
                      protoFileViewModel.removeProtoFile(projectIndex, value);
                    } else {
                      protoFileViewModel.removeImportPath(projectIndex, value);
                    }
                    Log.d("onPressed");
                  },
                  icon: const Icon(
                    Icons.remove,
                  ))),
          Expanded(child: Text(value)),
        ]));
  }

  Future<void> _pickProtoFile(
      int projectIndex, ProtoFileViewModel viewModel) async {
    FilePickerResult? result = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: ['proto'],
    );
    if (result != null) {
      var file = result.files.single.path;
      if (file != null) {
        viewModel.addProtoFile(projectIndex, file);
      } else {
        Log.w('_pickProtoFile file is null');
      }
    }
  }

  Future<String?> _pickImportPath(
      int projectIndex, ProtoFileViewModel viewModel) async {
    String? path = await FilePicker.platform.getDirectoryPath();
    if (path != null) {
      viewModel.addImportPath(projectIndex, path);
    } else {
      Log.e('_pickImportPath path is null');
    }
    return null;
  }
}
